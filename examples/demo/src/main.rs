#![no_std]
#![no_main]

use psx::constants::*;
use psx::gpu::primitives::Sprt;
use psx::gpu::{TexCoord, Vertex};
use psx::include_array;
use psx::{Framebuffer, TIM};

#[no_mangle]
unsafe fn main() {
    let mut fb = Framebuffer::default();
    let mut ferris = include_array!("../ferris.tim");
    let ferris = fb.load_tim(TIM::new(&mut ferris).expect("The TIM file is invalid"));
    let mut pos = Vertex::new((0, 8));
    let mut sprt = Sprt::new();
    sprt.set_color(WHITE.into())
        .set_offset(pos)
        .set_size(Vertex::new((128, 128)))
        .set_clut(ferris.clut.expect("The TIM file didn't have a CLUT"))
        .set_tex_coord(TexCoord { x: 0, y: 0 });
    loop {
        pos += Vertex::new((1, 1));
        sprt.set_offset(pos);
        fb.gp0.send_command(&sprt);
        fb.draw_sync();
        fb.swap();
        delay(100000);
    }
}

fn delay(n: usize) {
    for _ in 0..n {
        unsafe {
            core::ptr::read_volatile(0 as *const u32);
        }
    }
}
