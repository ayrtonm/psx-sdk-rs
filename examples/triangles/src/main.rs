#![no_std]
#![no_main]

use core::mem::size_of;

use psx::bios;
use psx::dma;
use psx::general::{reset_graphics, enable_display, vsync};
use psx::framebuffer::Framebuffer;
use psx::printer::Printer;
use psx::graphics::buffer::DoubleBuffer;
use psx::graphics::packet::Packet;
use psx::graphics::primitive::PolyG3;
use psx::graphics::ot::DoubleOT;
use psx::gpu::{Vertex, Color};
use psx::workarounds::UnwrapUnchecked;

#[no_mangle]
fn main(mut gpu_dma: dma::gpu::CHCR) {
    let gpu_dma = &mut gpu_dma;
    reset_graphics(gpu_dma);
    let mut fb = Framebuffer::new(0, (0, 240), (320, 240), None, gpu_dma);

    let mut p = Printer::new(0, 0, (320, 8), None);
    p.load_font(gpu_dma);

    bios::srand(0xdead_beef);

    const MAX_TRIANGLES: usize = 200;
    const BUF: usize = MAX_TRIANGLES * (size_of::<Packet<PolyG3>>() / 4);
    let mut buffer = DoubleBuffer::<BUF>::new();
    let mut poly_g3s = buffer.poly_g3_array::<MAX_TRIANGLES>().unwrap_unchecked();
    let mut ot = DoubleOT::default();

    for poly_g3 in &mut poly_g3s {
        ot.insert(poly_g3, 0);
    }
    ot.swap();
    buffer.swap();
    for poly_g3 in &mut poly_g3s {
        ot.insert(poly_g3, 0);
    }

    enable_display();
    let mut i = 0;
    loop {
        let transfer = gpu_dma.send_list(ot.swap());
        for poly_g3 in &mut poly_g3s {
            poly_g3.set_vertices(rand_triangle()).set_colors(rand_colors());
        }
        buffer.swap();
        transfer.wait();
        p.print(b"drawing frame {}", [i], gpu_dma);
        p.reset();
        i += 1;
        vsync();
        fb.swap(gpu_dma);
    }
}

fn rand_vertex() -> Vertex {
    (bios::rand() % 320, bios::rand() % 240).into()
}

fn rand_triangle() -> [Vertex; 3] {
    [rand_vertex(), rand_vertex(), rand_vertex()]
}

fn rand_color() -> Color {
    Color::rgb(bios::rand() as u8, bios::rand() as u8, bios::rand() as u8)
}

fn rand_colors() -> [Color; 3] {
    [rand_color(), rand_color(), rand_color()]
}
