#![no_std]
#![no_main]
#![feature(inline_const)]

// TODO: Change this demo from floating point to 16-bit fixed point

mod cube;
use cube::{Cube, Plane};
use psx::constants::*;
use psx::dma;
use psx::gpu::primitives::PolyGT4;
use psx::gpu::{link_list, Packet, TexCoord, Vertex};
use psx::include_words;
use psx::sys::gamepad::Gamepad;
use psx::{Framebuffer, TIM};

// We don't really need a heap for this demo, but the `sort_by_key` function is
// not available unless we have one (even if it doesn't use it). It seems that
// allocations never happen since the slice we're sorting is small so we can
// safely specify the BIOS malloc/free to avoid pulling in needless
// dependencies.
psx::sys_heap! {
    unsafe {
        core::slice::from_raw_parts_mut(DATA_CACHE, DATA_CACHE_LEN)
    }
}

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

    // Each `Packet` has a header with the physical address of the next `Packet`.
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
    let colors = [MINT, VIOLET, INDIGO, ORANGE, AQUA, LIME];
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
            .set_clut(ferris.clut.unwrap()) // This panics if ferris.tim doesn't have a CLUT
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
            .set_clut(ferris.clut.unwrap()) // This panics if ferris.tim doesn't have a CLUT
            .set_tex_coords([(0, 0), (0, 85), (128, 0), (128, 85)].map(|(x, y)| TexCoord { x, y }));
    }

    let cube = Cube::new();
    let mut theta = 0.0;
    let mut phi = 0.0;

    let mut dtheta = 0.0;
    let mut dphi = 0.0;

    let friction = 0.1;

    // Create a buffer for gamepad input. This uses a static buffer that will be
    // managed by the BIOS until `pad` is dropped.
    let mut pad = Gamepad::new();

    let mut iterations = 0;

    loop {
        theta += dtheta;
        phi += dphi;
        let eps = 0.07;
        if dtheta > eps {
            dtheta -= friction;
        } else if dtheta < -eps {
            dtheta += friction;
        }
        if dphi > eps {
            dphi -= friction;
        } else if dphi < -eps {
            dphi += friction;
        }
        // Poll player 1's controller and iterate through the pressed buttons
        for button in pad.poll_p1() {
            // Check what button was pressed
            match button {
                CROSS => dtheta += 0.12,
                TRIANGLE => dtheta -= 0.12,
                SQUARE => dphi += 0.12,
                CIRCLE => dphi -= 0.12,
                _ => (),
            }
        }
        // We want loadable executables to be able to exit at some point
        iterations += 1;
        if iterations > 500 {
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
            let mut new_cube = cube.faces.map(|plane| plane.rx(theta).ry(phi));
            new_cube.sort_by_key(|plane| {
                let mut res = 0.;
                for p in plane.points {
                    res += p.z * 10.;
                }
                -res as i16
            });

            // Update the vertices of the polygons not being displayed
            for n in 0..6 {
                let new_vertices = project_plane(new_cube[n]);
                draw_cube[n].contents.set_vertices(new_vertices);
                for c in draw_cube[n].contents.get_colors_mut() {
                    *c += ORANGE / 16;
                }
            }
        });
        fb.draw_sync();
        fb.wait_vblank();
        fb.dma_swap(&mut dma_gpu);
    }
}

fn project_plane(face: Plane) -> [Vertex; 4] {
    let scale = 180.;
    face.points.map(|p3| {
        let x = scale * p3.x / (p3.z + 2.);
        let y = scale * p3.y / (p3.z + 2.);

        Vertex(x as i16, y as i16) + Vertex(160, 120)
    })
}
