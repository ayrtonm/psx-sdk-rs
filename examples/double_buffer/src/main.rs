#![no_std]
#![no_main]

use psx::dma;
use psx::gpu::colors::*;
use psx::gpu::primitives::PolyF4;
use psx::gpu::{Color, Packet, Vertex};
use psx::println;
use psx::sys::gamepad::buttons::*;
use psx::sys::gamepad::{Gamepad, PadType};
use psx::{draw_sync, enable_vblank, f16, link_list, vsync, Framebuffer, Vi};

const BUF0: Vertex = Vertex(0, 0);
const BUF1: Vertex = Vertex(0, 240);
const RES: Vertex = Vertex(320, 240);

// `sort_by_key` is only available when linking alloc so let's create a heap in
// the 1KB data cache
psx::heap! {
    // SAFETY: The data cache isn't used by anything else
    unsafe {
        psx::data_cache()
    }
}

#[no_mangle]
fn main() -> Result<(), &'static str> {
    // Initializes the GPU and creates a Framebuffer with a white background
    let mut fb = Framebuffer::new(BUF0, BUF1, RES, Some(WHITE))?;
    // Initializes the GPU DMA channel
    let mut gpu_dma = dma::GPU::new();
    // The BIOS Gamepad wrapper needs pinned buffers for the controller data so
    // it must be created outside of `Gamepad::new`
    let mut buf0 = [0; Gamepad::BUFFER_SIZE];
    let mut buf1 = [0; Gamepad::BUFFER_SIZE];
    let pad = Gamepad::new(&mut buf0, &mut buf1)?;

    // Make a list of planes defined by 4 3D integer vectors (i.e. [Vi; 4])
    // which define a unit cube.
    let xy = [Vi::ZERO, Vi::X, Vi::Y, Vi::X + Vi::Y];
    let yz = [Vi::ZERO, Vi::Y, Vi::Z, Vi::Y + Vi::Z];
    let xz = [Vi::ZERO, Vi::X, Vi::Z, Vi::Z + Vi::X];
    let unit_cube = [
        (xy, RED),
        (yz, GREEN),
        (xz, BLUE),
        (xy.map(|v| v + Vi::Z), YELLOW),
        (yz.map(|v| v + Vi::X), CYAN),
        (xz.map(|v| v + Vi::Y), VIOLET),
    ];
    // Define the initial position and angles of the cube
    let mut pos = Vi(0, 0, 0x2000);
    let mut theta = f16(0);
    let mut phi = f16(0);

    // Define the data that will be sent to the GPU. `PolyF4` is a four-point
    // monochrome polygon and they're wrapped in a `Packet` to allow sending
    // them through the DMA channel in a linked list. Since we're double
    // buffering we need two `Packet<PolyF4>`s per cube face for a total of 12.
    let mut quads = [PolyF4::new(); 12].map(|q| Packet::new(q));
    // quads[0..6] is the first cube and quads[6..12] is the second one
    let (cube_a, cube_b) = quads.split_at_mut(6);
    // Link the packets (face polygons) in each slice (cube) as follows
    // cube_a[0] -> cube_a[1] -> cube_a[2] ... -> cube_a[5] -> TERMINATE_LIST
    link_list(cube_a);
    // cube_a[6] -> cube_a[7] -> cube_a[8] ... -> cube_a[11] -> TERMINATE_LIST
    link_list(cube_b);
    let mut display_a = true;

    // Wait until the BIOS initializes the Gamepad data
    while pad.info() == PadType::Unknown {}
    let mut controller = pad.info();
    println!("{:?} controller connected", controller);
    enable_vblank();
    loop {
        // Check if we connected a new controller
        let new_controller = pad.info();
        if new_controller != controller {
            controller = new_controller;
            println!("Switched to the {:?} controller", new_controller);
        }

        let (cube_a, cube_b) = quads.split_at_mut(6);
        // Decide which cube will be displayed and which will have its coordinates updated
        let (display_list, draw_list) = if display_a {
            (cube_a, cube_b)
        } else {
            (cube_b, cube_a)
        };
        // Start sending the display cube to the GPU
        gpu_dma.send_list_and(display_list, || {
            // Calculate the new coordinates of the other cube while the DMA transfer is ongoing.
            // More specifically the CPU executes this closure between the Packet transfers, NOT
            // concurrently with the DMA. If the closure terminates before the DMA transfer ends
            // the CPU hangs until the transfer is done.

            // Scale up the unit cube defined above, rotate it about its center and shift its position.
            let mut cube = unit_cube.map(|(face, color)| {
                let scale = 0x1000;
                let center = (Vi::X + Vi::Y + Vi::Z) * scale / 2;
                (
                    face.map(|vi| {
                        (vi * scale).rotate_x(theta, center).rotate_y(phi, center) - center + pos
                    }),
                    color,
                )
            });

            // Sort the faces of the cube based on the average of the z-coordinates of their vertices.
            // This ensures that the `PolyF4`s in the draw list are ordered from farthest to closest.
            cube.sort_by_key(avg_z);

            for n in 0..6 {
                let (face, color) = cube[n];
                // Project each 3D integer vector onto the 2D screen
                let projected_quad = face.map(project_vector);
                // Update the contents of each `PolyF4`s in the draw list.
                draw_list[n]
                    .contents
                    .set_vertices(projected_quad)
                    .set_color(color);
            }
            // Swap the display list and draw list next time
            display_a = !display_a;

            // Update the cube's position and angles based on the controller
            poll_controller(&pad, &mut pos, &mut theta, &mut phi);
        })?;
        // Wait until the GPU processes all the `PolyF4`s
        draw_sync();
        // Wait until vertical blank
        vsync();
        // Swap the frambuffer using the GPU DMA channel
        fb.swap(Some(&mut gpu_dma))?;
    }
}

fn project_vector(Vi(x, y, z): Vi) -> Vertex {
    // Divide two i16's to produce a signed 16-bit fixed point result
    let ex = f16::div(x, z);
    let ey = f16::div(y, z);
    let bx = ex * 120;
    let by = ey * 120;
    let res = Vertex(bx + 160, by + 120);
    res
}

fn avg_z((face, _): &([Vi; 4], Color)) -> i32 {
    let mut res = 0;
    for Vi(_, _, z) in *face {
        res += i32::from(z) / 4;
    }
    -res
}

fn poll_controller(pad: &Gamepad, pos: &mut Vi, theta: &mut f16, phi: &mut f16) {
    let buttons = pad.poll();

    let pos_step = 0x100;
    if buttons.pressed(UP) {
        *pos -= Vi::Y * pos_step;
    } else if buttons.pressed(DOWN) {
        *pos += Vi::Y * pos_step;
    }
    if buttons.pressed(LEFT) {
        *pos -= Vi::X * pos_step;
    } else if buttons.pressed(RIGHT) {
        *pos += Vi::X * pos_step;
    }
    let theta_step = f16(0x_200);
    if buttons.pressed(CROSS) {
        *theta += theta_step;
    } else if buttons.pressed(TRIANGLE) {
        *theta -= theta_step;
    }
    let phi_step = f16(0x_200);
    if buttons.pressed(SQUARE) {
        *phi += phi_step;
    } else if buttons.pressed(CIRCLE) {
        *phi -= phi_step;
    }
}
