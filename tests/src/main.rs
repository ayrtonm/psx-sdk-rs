#![no_std]
#![no_main]

use psx::general::*;
use psx::dma;
use psx::framebuffer::Framebuffer;
use psx::gpu::Color;

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

    loop {
        vsync();
        fb.swap(&mut gpu_dma);
    }
}
