#![no_std]
#![no_main]

use psx::{dprintln, Framebuffer};

#[no_mangle]
fn main() {
    let buf0 = (0, 0);
    let buf1 = (0, 240);
    let res = (320, 240);
    let txt_offset = (0, 8);
    let mut fb = Framebuffer::new(buf0, buf1, res, None).unwrap();
    let font = fb.load_default_font();
    let mut txt = font.new_text_box(txt_offset, res);
    loop {
        txt.reset();
        dprintln!(txt, "Hello, world!");
        fb.draw_sync();
        fb.wait_vblank();
        fb.swap();
    }
}
