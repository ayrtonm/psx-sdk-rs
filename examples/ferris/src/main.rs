#![no_std]
#![no_main]
#![feature(inline_const)]

use psx::constants::*;
use psx::gpu::primitives::*;
use psx::gpu::{link_list, Packet, TexCoord, Vertex};
use psx::include_tim;
use psx::math::{f16, rotate_x, rotate_y, rotate_z};
use psx::{dma, dprintln, Framebuffer};

// We don't really need a heap for this demo, but the `sort_by_key` function is
// in the `alloc` crate so it's unavailable unless we have a heap (even if it
// never uses it). It seems that allocations never happen since the slice we're
// sorting is small so we can safely specify the BIOS malloc/free to avoid
// pulling in needless dependencies.
psx::sys_heap!(0 bytes);

#[no_mangle]
fn main() {
    let mut fb = Framebuffer::default();
    let mut txt = fb.load_default_font().new_text_box((0, 8), (320, 240));
    let mut gpu_dma = dma::GPU::new();

    // This is a `TIM` which references the file embedded in the executable's .data
    // section
    let ferris_tim = include_tim!("../ferris.tim");
    // This represents the loaded TIM file and contains the TexPage and Clut (if
    // any)
    let loaded_tim = fb.load_tim(ferris_tim);

    let mut polygons = [const { Packet::new(PolyGT4::new()) }; 12];

    let polygons = &mut polygons;

    link_list(&mut polygons[0..6]);
    link_list(&mut polygons[6..12]);

    let tex_coords = [(0, 0), (0, 85), (128, 0), (128, 85)].map(|(x, y)| TexCoord { x, y });
    for p in polygons.iter_mut() {
        p.contents
            .set_tex_page(loaded_tim.tex_page)
            .set_clut(loaded_tim.clut.unwrap())
            .set_tex_coords(tex_coords);
    }

    let vertices = [
        ([1, 1, -1], VIOLET),
        ([1, -1, -1], INDIGO),
        ([1, 1, 1], ORANGE),
        ([1, -1, 1], LIME),
        ([-1, 1, -1], YELLOW),
        ([-1, -1, -1], CYAN),
        ([-1, 1, 1], MINT),
        ([-1, -1, 1], PINK),
    ]
    .map(|(v, c)| (v.map(|e| f16::from_int(e) / f16(0x2_000)), c));
    let mut faces = [
        [0, 4, 2, 6],
        [3, 2, 7, 6],
        [7, 6, 5, 4],
        [5, 1, 7, 3],
        [1, 0, 3, 2],
        [5, 4, 1, 0],
    ];

    let mut swapped = false;

    let mut theta = FRAC_PI_8 * f16(0x_800);
    let mut phi = FRAC_PI_8 * f16(0x_400);
    let mut psi = FRAC_PI_8 * f16(0x_200);

    let vel = FRAC_PI_8 * f16(0x_060);

    loop {
        theta += vel * f16(0x2_000);
        phi += vel * f16(0x4_000);
        psi += vel;
        txt.reset();
        dprintln!(txt, "theta: {:#x?}", theta.0);
        dprintln!(txt, "phi: {:#x?}", phi.0);
        dprintln!(txt, "psi: {:#x?}", psi.0);

        // We want some way to return to the loader if this is a loadable executable
        if cfg!(feature = "loadable_exe") {
            if psi > f16(0x7_000) {
                return
            }
        }

        swapped = !swapped;
        let (a, b) = polygons.split_at_mut(6);
        let (draw_poly, disp_poly) = if swapped { (a, b) } else { (b, a) };
        gpu_dma.send_list_and(disp_poly, || {
            let rotated_vertices =
                vertices.map(|(v, c)| (rotate_z(rotate_x(rotate_y(v, theta), phi), psi), c));

            faces.sort_by_key(|face| {
                let points = face.map(|i| rotated_vertices[i].0);
                let mut res = 0;
                for [_, _, z] in points {
                    res += z.0 >> 2;
                }
                -res
            });
            for n in 0..6 {
                let projected_vertices = project_face(faces[n].map(|i| rotated_vertices[i].0));
                draw_poly[n]
                    .contents
                    .set_vertices(projected_vertices)
                    .set_colors(faces[n].map(|i| rotated_vertices[i].1));
            }
        });
        fb.draw_sync();
        fb.wait_vblank();
        fb.dma_swap(&mut gpu_dma);
    }
}

fn project_face(face: [[f16; 3]; 4]) -> [Vertex; 4] {
    face.map(|[x, y, z]| {
        let scale = 16;
        let xp = x / (z + f16(0x3_000));
        let yp = y / (z + f16(0x3_000));
        Vertex(xp.0 / scale, yp.0 / scale) + Vertex(160, 120)
    })
}
