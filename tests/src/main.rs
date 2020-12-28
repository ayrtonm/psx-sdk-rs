#![no_std]
#![no_main]

use psx::compatibility::*;
use psx::dma;
use psx::framebuffer::Framebuffer;
use psx::gpu::Color;

#[no_mangle]
fn main(mut gpu_dma: dma::gpu::CHCR) {
    ResetGraph(0, &mut gpu_dma);
    let mut fb = Framebuffer::new(
        (0, 0),
        (0, 240),
        (320, 240),
        Some(Color::INDIGO),
        &mut gpu_dma,
    );

    loop {
        DrawSync(0, &gpu_dma);
        //VSync(0);
        fb.swap(&mut gpu_dma);
        SetDispMask(1);
    }
}
