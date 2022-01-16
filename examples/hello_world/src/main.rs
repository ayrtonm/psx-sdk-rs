#![no_std]
#![no_main]

use psx::gpu::colors::*;
use psx::gpu::Vertex;
use psx::{dprintln, Font, Framebuffer};

#[no_mangle]
fn main() {
    let buf0 = Vertex(0, 0);
    let buf1 = Vertex(0, 240);
    let res = Vertex(320, 240);
    // Creating a new Framebuffer will `Err` if the resolution isn't valid.
    let mut fb = Framebuffer::new(buf0, buf1, res).expect("Resolution is valid");
    let offset = Vertex(0, 8);
    let mut text_box = Font::default().text_box(offset);
    dprintln!(text_box, "Hello, world!");
    text_box.newline();
    dprintln!(text_box, "I can write in all these colors");
    let colors = [
        RED, ORANGE, YELLOW, LIME, GREEN, MINT, CYAN, AQUA, BLUE, INDIGO, VIOLET, PINK,
    ];
    for c in colors {
        text_box.change_color(c);
        dprintln!(text_box, "{:?}", c);
    }
    // Swap the frambuffers with the CPU instead of the GPU DMA channel
    fb.swap(None);
    loop {}
}
