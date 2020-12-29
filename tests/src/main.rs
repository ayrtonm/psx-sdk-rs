#![no_std]
#![no_main]

use core::mem::size_of;
use psx::graphics::packet::Packet;
use psx::graphics::primitive::PolyF3;

use psx::bios;
use psx::dma;
use psx::framebuffer::Framebuffer;
use psx::general::*;
use psx::gpu::{Color, Vertex};
use psx::graphics::buffer::DoubleBuffer;
use psx::graphics::ot::DoubleOT;
use psx::printer::Printer;

#[no_mangle]
fn main(mut gpu_dma: dma::gpu::CHCR) {
    reset_graphics(&mut gpu_dma);
    let mut fb = Framebuffer::new(
        (0, 0),
        (0, 240),
        (320, 240),
        Some(Color::INDIGO),
        &mut gpu_dma,
    );
    enable_display();

    // The printer's buffer size doesn't really matter as long as it can hold at least one sprite
    let mut printer = Printer::new(0, 0, (320, 240), Some(Color::WHITE));
    printer.load_font(&mut gpu_dma);

    bios::srand(0xdead_beef);

    const NUM: usize = 100;
    const BUF: usize = NUM * (size_of::<Packet<PolyF3>>() / 4);
    let buffer = DoubleBuffer::<BUF>::new();
    let mut ot = DoubleOT::default();
    let mut trs = buffer.poly_f3_array::<NUM>().unwrap();

    for tr in &mut trs {
        ot.insert(tr, 0);
    }
    buffer.swap();
    ot.swap();
    for tr in &mut trs {
        ot.insert(tr, 0);
    }

    loop {
        let transfer = gpu_dma.send_list(&ot);
        buffer.swap();
        for tr in &mut trs {
            tr.set_color(random_color()).set_vertices(random_triangle());
        }
        transfer.wait().swap();
        printer.print(
            b"\nHello world! {} {0}",
            [0xdead_beef, 0xffff_0000],
            &mut gpu_dma,
        );
        printer.reset();
        vsync();
        fb.swap(&mut gpu_dma);
    }
}

fn random_vertex() -> Vertex {
    (bios::rand() % 320, bios::rand() % 240).into()
}

fn random_triangle() -> [Vertex; 3] {
    [random_vertex(), random_vertex(), random_vertex()]
}

fn random_color() -> Color {
    (bios::rand() as u8, bios::rand() as u8, bios::rand() as u8).into()
}
