#![no_std]
#![no_main]

use psx::{Font, Framebuffer, dprintln};
use psx::gpu::Vertex;

#[no_mangle]
fn main() {
    let buf0 = Vertex(0, 0);
    let buf1 = Vertex(0, 240);
    let res = Vertex(320, 240);
    let mut fb = Framebuffer::new(buf0, buf1, res, None).expect("Resolution is valid");
    let offset = Vertex(0, 8);
    let mut text_box = Font::default().text_box(offset, None);
    dprintln!(text_box, "Hello, world!");
    fb.swap(None).expect("Swapping the framebuffers will never fail without DMA");
    loop {}
}
