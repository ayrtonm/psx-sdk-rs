#![no_std]
#![no_main]
#![feature(inline_const, const_mut_refs)]

use psx::constants::*;
use psx::gpu::primitives::*;
use psx::gpu::{link_list, Color, Packet, TexCoord, Vertex};
use psx::include_obj;
use psx::math::{f16, rotate_x, rotate_y, rotate_z};
use psx::sys::rng::Rng;
use psx::{dma, println, Framebuffer};

psx::sys_heap!(0 bytes);

#[no_mangle]
fn main() {
    let mut fb = Framebuffer::default();
    fb.set_bg_color(WHITE);
    let mut gpu_dma = dma::GPU::new();

    let donut = include_obj!("../../../psx/test_files/torus.obj");

    // Assign a random color to each vertex
    let colored_vertices = donut.vertices.map(|v| (v, rand_color()));

    // Make two sets of polygons for double-buffering
    // `for_each_face` creates an array of length `quads.len + tris.len`, but
    // torus.obj has no tris so all the packets can contain PolyG4s. Working with
    // arrays with only one type of polygon is restrictive, but makes things much
    // easier. See the monkey demo for an example of how to work with multiple types
    // of polygons
    let mut polys_a = donut.for_each_face(|| Packet::new(PolyG4::new()));
    let mut polys_b = donut.for_each_face(|| Packet::new(PolyG4::new()));

    // Link the packets in each set of polygons
    link_list(&mut polys_a);
    link_list(&mut polys_b);

    let mut swapped = false;

    let mut theta = FRAC_PI_8 * f16(0x_800);
    let mut phi = FRAC_PI_8 * f16(0x_400);
    let mut psi = FRAC_PI_8 * f16(0x_200);

    let vel = FRAC_PI_8 * f16(0x_0F0);

    loop {
        theta += vel * f16(0x2_000);
        phi += vel * f16(0x4_000);
        psi += vel;

        swapped = !swapped;
        let (draw_poly, disp_poly) = if swapped {
            (&mut polys_a, &mut polys_b)
        } else {
            (&mut polys_b, &mut polys_a)
        };
        gpu_dma.send_list_and(disp_poly, || {
            // Rotate all vertices and keep track of the colors
            let rotated_vertices = colored_vertices
                .map(|(v, c)| (rotate_z(rotate_x(rotate_y(v, theta), phi), psi), c));

            // Sort the donut faces by the average z of their rotated vertices
            donut.faces.quads.sort_by_key(|face| {
                let points = face.map(|i| rotated_vertices[i as usize].0);
                let mut res = 0;
                for [_, _, z] in points {
                    res += z.0 >> 2;
                }
                -res
            });
            for n in 0..draw_poly.len() {
                // Project the vertices onto the screen
                let projected_vertices =
                    project_face(donut.faces.quads[n].map(|i| rotated_vertices[i as usize].0));
                // Update the vertices and colors of each polygon in the draw list
                draw_poly[n]
                    .contents
                    .set_vertices(projected_vertices)
                    .set_colors(donut.faces.quads[n].map(|i| rotated_vertices[i as usize].1));
            }
        });
        fb.draw_sync();
        fb.wait_vblank();
        fb.dma_swap(&mut gpu_dma);
    }
}

fn project_face(face: [[f16; 3]; 4]) -> [Vertex; 4] {
    face.map(|[x, y, z]| {
        let scale = 32;
        let xp = x / (z + f16(0x1_800));
        let yp = y / (z + f16(0x1_800));
        Vertex(xp.0 / scale, yp.0 / scale) + Vertex(160, 120)
    })
}
fn rand_color() -> Color {
    static mut RNG: Option<Rng> = None;
    unsafe {
        if RNG.is_none() {
            RNG = Some(Rng::new(0xdeadbeef));
        };
        RNG.as_mut()
            .map(|rng| Color::new(rng.rand(), rng.rand(), rng.rand()))
            .unwrap()
    }
}
