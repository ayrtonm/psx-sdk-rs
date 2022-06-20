#![no_std]
#![no_main]
#![feature(inline_const, const_mut_refs)]

use psx::constants::*;
use psx::gpu::primitives::*;
use psx::gpu::{link_list, Color, Packet, Vertex};
use psx::hw::gpu::GP0Command;
use psx::include_obj;
use psx::math::{f16, rotate_x, rotate_y, rotate_z};
use psx::sys::rng::Rng;
use psx::{dma, Framebuffer};

psx::sys_heap!(0 bytes);

#[derive(Debug, Clone, Copy)]
enum Face {
    Tri([u16; 3]),
    Quad([u16; 4]),
}

// An untagged union which is used to simplify sorting the polygon list (which
// is a [Packet<PolyF>; N]). Note that creating an enum with Packet<PolyF3> and
// Packet<PolyF4> is a valid alternative since the Packet contents would be
// valid in both cases, but that makes linking packets more cumbersome since we
// couldn't use link_packets.
#[repr(C)]
union PolyF {
    tri: PolyF3,
    quad: PolyF4,
}

// This trait allows safe access to the contents of a Packet with a union of two
// polygons
pub trait SafeUnionAccess {
    fn as_quad(&mut self) -> &mut PolyF4;
    fn as_tri(&mut self) -> &mut PolyF3;
}
impl SafeUnionAccess for Packet<PolyF> {
    fn as_quad(&mut self) -> &mut PolyF4 {
        // SAFETY: We resize the packet to hold a PolyF4 and reset the polygon's command
        // to ensure that the union's PolyF4 is in a valid state when we access it
        unsafe { self.resize::<PolyF4>().contents.quad.reset_cmd() }
    }
    fn as_tri(&mut self) -> &mut PolyF3 {
        // SAFETY: We resize the packet to hold a PolyF3 and reset the polygon's command
        // to ensure that the union's PolyF3 is in a valid state when we access it
        unsafe { self.resize::<PolyF3>().contents.tri.reset_cmd() }
    }
}

// Since the variants of PolyF are both GP0 commands, this is fine. However, we
// must make sure to resize the packet when changing variants
impl GP0Command for PolyF {}

#[no_mangle]
fn main() {
    let mut fb = Framebuffer::default();
    let mut gpu_dma = dma::GPU::new();

    // To display this correctly, we need to sort the faces before drawing just like
    // in the previous examples. This is a problem for this particular .obj since it
    // contains both tris and quads which are laid out in two separate arrays
    // `monkey.faces.tris` and `monkey.faces.quads` so we can't use sort_by_key
    // directly. To get around this we merge them into a new array below.
    let monkey = include_obj!("../../../psx/test_files/monkey.obj");

    // For each face, create a Face::Quad if it's a quad or a Face::Tri if it's a
    // tri. This array is used for sorting, and not sent to the GPU DMA so the
    // memory layout doesn't matter which allows us to use an enum. Also assign
    // a random color to each vertex
    let mut faces = monkey.map_faces(
        |q| (Face::Quad(q), rand_color()),
        |t| (Face::Tri(t), rand_color()),
    );

    // Define functions to initialize the polygons
    let init_quad_poly = |q: [u16; 4]| -> Packet<PolyF> {
        // Create a PolyF4 and initialize it like normal
        let mut quad = PolyF4::new();
        quad.set_vertices(q.map(|i| monkey.vertices[i as usize]).map(project_point));
        // Since the array must hold both quads and tris, each packet contains an
        // untagged union with the two types of polygons as its variants. The packet
        // size is initialized to size of PolyF which is the same as size of PolyF4
        // since that's the largest variant.
        Packet::new(PolyF { quad })
    };

    let init_tri_poly = |t: [u16; 3]| -> Packet<PolyF> {
        // Create a PolyF3 and initialize it like normal
        let mut tri = PolyF3::new();
        tri.set_vertices(t.map(|i| monkey.vertices[i as usize]).map(project_point));
        let mut p = Packet::new(PolyF { tri });
        // Since the untagged union was initialized to a tri, we have to shorten the
        // size of the packet.
        p.resize::<PolyF3>();
        p
    };

    // Create two sets of polygons. Note that polys_* are arrays with both quads
    // and tris
    let mut polys_a = monkey.map_faces(init_quad_poly, init_tri_poly);
    let mut polys_b = monkey.map_faces(init_quad_poly, init_tri_poly);

    // Link the packets in each set of polygons
    link_list(&mut polys_a);
    link_list(&mut polys_b);

    let mut swapped = false;

    let mut theta = PI;
    let mut phi = f16(0);
    let mut psi = PI;

    let vel = FRAC_PI_8 * f16(0x_0F0);

    loop {
        psi += vel * f16(0x4_000);
        phi += vel * f16(0x2_000);
        theta += vel;

        swapped = !swapped;
        let (draw_poly, disp_poly) = if swapped {
            (&mut polys_a, &mut polys_b)
        } else {
            (&mut polys_b, &mut polys_a)
        };
        gpu_dma.send_list_and(disp_poly, || {
            // Rotate all vertices and keep track of the colors
            let rotated_vertices = monkey
                .vertices
                .map(|v| rotate_z(rotate_x(rotate_y(v, theta), phi), psi));

            // Sort the monkey faces by the average z of their rotated vertices. Note that
            // the average z computation is different for quads and tris.
            faces.sort_by_key(|(face, _)| {
                let mut res = 0;
                match face {
                    Face::Quad(q) => {
                        let points = q.map(|i| rotated_vertices[i as usize]);
                        for [_, _, z] in points {
                            res += z.0 / 4;
                        }
                    },
                    Face::Tri(t) => {
                        let points = t.map(|i| rotated_vertices[i as usize]);
                        for [_, _, z] in points {
                            res += z.0 / 3;
                        }
                    },
                }
                -res
            });
            for n in 0..draw_poly.len() {
                match faces[n].0 {
                    Face::Quad(q) => {
                        // Project the vertices onto the screen
                        let projected_vertices =
                            q.map(|i| project_point(rotated_vertices[i as usize]));
                        // Update the vertices and colors of each polygon in the draw list
                        draw_poly[n]
                            .as_quad()
                            .set_vertices(projected_vertices)
                            .set_color(faces[n].1);
                    },
                    Face::Tri(t) => {
                        let projected_vertices =
                            t.map(|i| project_point(rotated_vertices[i as usize]));
                        draw_poly[n]
                            .as_tri()
                            .set_vertices(projected_vertices)
                            .set_color(faces[n].1);
                    },
                }
            }
        });
        fb.draw_sync();
        fb.wait_vblank();
        fb.dma_swap(&mut gpu_dma);
    }
}

fn project_point([x, y, z]: [f16; 3]) -> Vertex {
    let scale = 32;
    let xp = x / (z + f16(0x1_800));
    let yp = y / (z + f16(0x1_800));
    Vertex(xp.0 / scale, yp.0 / scale) + Vertex(160, 120)
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
