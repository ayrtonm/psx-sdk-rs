#![no_std]
#![no_main]

use psx::dma;
use psx::framebuffer::Framebuffer;
use psx::general::*;
use psx::gpu::Color;
use psx::graphics::buffer::Buffer;
use psx::graphics::ot::OT;

#[no_mangle]
fn main(mut gpu_dma: dma::gpu::CHCR) {
    reset_graphics();
    let mut fb = Framebuffer::new(
        (0, 0),
        (0, 240),
        (320, 240),
        Some(Color::WHITE),
        &mut gpu_dma,
    );
    enable_display();

    let buffer = Buffer::<256>::new();
    let mut ot = OT::default();
    let tr = buffer.PolyF3().unwrap();
    tr.set_color(Color::GREEN)
        .set_vertices([(0, 0), (50, 0), (0, 50)]);
    ot.insert(tr, 0);

    loop {
        gpu_dma.send_list(&ot);
        vsync();
        fb.swap(&mut gpu_dma);
    }
}
