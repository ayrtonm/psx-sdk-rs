#![no_std]
#![no_main]
#![feature(inline_const, const_fn_floating_point_arithmetic)]

mod cube;
use core::mem::MaybeUninit;
use cube::{Cube, Plane};
use psx::constants::*;
use psx::dma;
use psx::gpu::primitives::PolyGT4;
use psx::gpu::{link_list, Packet, TexCoord, Vertex};
use psx::include_words;
use psx::sys::gamepad::Gamepad;
use psx::{Framebuffer, TIM};

#[no_mangle]
fn main() {
    let mut fb: Framebuffer = Framebuffer::default();
    let mut dma_gpu: dma::GPU = dma::GPU::new();

    // Get a mutable reference to the TIM in the executable's .data section
    let tim = include_words!("../ferris.tim");
    // Validate the TIM and load it into VRAM, getting a `LoadedTIM`
    let ferris = fb.load_tim(TIM::new(tim).expect("The TIM file is invalid"));

    // Allocate stack space for the polygons that will be sent to the GPU. To take
    // advantage of double-buffering we allocate enough polygons for 2 cubes. Each
    // cube consists of 6 `PolyGT4`s (i.e. 2-dimensional 4-point Gouraud-shaded
    // textured polygons).
    //
    // Sending polygons to the GPU via the CPU is slow so let's use DMA. To do so
    // we wrap each `PolyGT4` in a DMA `Packet`. Each set of 6 `Packet`s forms a
    // linked list which the DMA controller will follow to decide what to send to
    // the GPU. Also since `Packet<T>` does not implement Copy, we have to use the
    // inline const feature.
    let mut polygon_a = [const { Packet::new(PolyGT4::new()) }; 6];
    let mut polygon_b = [const { Packet::new(PolyGT4::new()) }; 6];

    // Each `Packet` has a header with the physicaly address of the next `Packet`.
    // This means that they must not move after we've initialized them. To do this
    // we use the following to pin `polygons` in-place on the stack.
    let polygon_a = &mut polygon_a;
    let polygon_b = &mut polygon_b;

    // Initialize each `[Packet<PolyGT4>; 6]`. The linked list will go from first to
    // last
    link_list(polygon_a);
    link_list(polygon_b);
    // Create a variable to determine which polygons is being drawn and which is
    // being displayed
    let mut swapped = false;

    // Initialize fixed properties of the `PolyGT4`s
    let colors = [MINT, YELLOW, INDIGO, ORANGE, AQUA, LIME];
    for n in 0..6 {
        // We can access the `Packet` contents (i.e. the `PolyGT4`) through the
        // `contents` field then use methods specific to `PolyGT4` to set its
        // properties.
        polygon_a[n]
            .contents
            .set_colors([
                colors[n],
                colors[(n + 1) % 6],
                colors[(n + 2) % 6],
                colors[(n + 3) % 6],
            ])
            .set_tex_page(ferris.tex_page)
            .set_clut(ferris.clut.expect("The TIM file didn't have a CLUT"))
            .set_tex_coords([(0, 0), (0, 85), (128, 0), (128, 85)].map(|(x, y)| TexCoord { x, y }));
        polygon_b[n]
            .contents
            .set_colors([
                colors[n],
                colors[(n + 1) % 6],
                colors[(n + 2) % 6],
                colors[(n + 3) % 6],
            ])
            .set_tex_page(ferris.tex_page)
            .set_clut(ferris.clut.expect("The TIM file didn't have a CLUT"))
            .set_tex_coords([(0, 0), (0, 85), (128, 0), (128, 85)].map(|(x, y)| TexCoord { x, y }));
    }

    let cube = Cube::new();
    let mut theta = 0.0;
    let mut phi = 0.0;
    let mut psi = 0.0;

    let mut dtheta = 0.0;
    let mut dphi = 0.0;
    let mut dpsi = 0.0;

    let friction = 0.1;

    // Create a buffer for gamepad input. This buffer will be managed by the BIOS
    // until `pad` is dropped so it must be pinned in-place.
    let mut buf = MaybeUninit::uninit();
    let mut pad = Gamepad::new(&mut buf);

    loop {
        theta += dtheta;
        phi += dphi;
        psi += dpsi;
        if dtheta > 0.0 {
            dtheta -= friction;
        } else if dtheta < 0.0 {
            dtheta += friction;
        }
        if dphi > 0.0 {
            dphi -= friction;
        } else if dphi < 0.0 {
            dphi += friction;
        }
        if dpsi > 0.0 {
            dpsi -= friction;
        } else if dpsi < 0.0 {
            dpsi += friction;
        }
        // Poll player 1's controller and iterate through the pressed buttons
        for button in pad.poll_p1() {
            // Check what button was pressed
            match button {
                TRIANGLE => dtheta += 0.5,
                CROSS => dtheta -= 0.5,
                SQUARE => dphi += 0.2,
                CIRCLE => dphi -= 0.2,
                _ => (),
            }
        }
        // We want loadable executables to be able to exit at some point
        if theta > 20.0 {
            //return
        }
        // Swap the polygons being drawn and those being displayed
        swapped = !swapped;
        let (draw_cube, display_cube) = if swapped {
            (&mut *polygon_a, &mut *polygon_b)
        } else {
            (&mut *polygon_b, &mut *polygon_a)
        };
        // Start sending one set of polygons
        dma_gpu.send_list_and(display_cube, || {
            // After the transfer starts, compute the positions of the other set of polygons
            let new_cube = cube.faces.map(|plane| plane.rx(theta).ry(phi).rz(psi));

            // Update the vertices of the polygons not being displayed
            for n in 0..6 {
                draw_cube[n].contents.set_vertices(as_vertices(new_cube[n]));
                for c in draw_cube[n].contents.get_colors_mut() {
                    *c += INDIGO / 32;
                }
            }
        });
        fb.draw_sync();
        fb.wait_vblank();
        fb.dma_swap(&mut dma_gpu);
    }
}

fn as_vertices(face: Plane) -> [Vertex; 4] {
    face.points.map(|p3| {
        Vertex::new((
            (p3.x * 64. + p3.z * 32.) as i16,
            (p3.y * 64. + p3.z * 32.) as i16,
        )) + Vertex(128, 128)
    })
}
