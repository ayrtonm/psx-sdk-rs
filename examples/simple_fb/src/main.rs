#![no_std]
#![no_main]

use core::cell::RefCell;

use libpsx::delay;
use libpsx::gpu::vertex::Vertex;
use libpsx::gpu::color::Color;
use libpsx::gpu::{Hres, Vres};
use libpsx::gpu::framebuffer::Framebuffer;

libpsx::exe!();

fn main(mut ctxt: Ctxt) {
    let draw_env = RefCell::new(ctxt.take_draw_env().expect("DrawEnv has been taken"));
    let display_env = RefCell::new(ctxt.take_display_env().expect("DisplaEnv has been taken"));
    let res = (Hres::H320, Vres::V240);
    let buf0 = (0, 0);
    let buf1 = (0, 240);
    let mut fb = Framebuffer::new(&draw_env, &display_env, buf0, buf1, res);
    let mut offset = 0;
    loop {
        offset += 1;
        delay(100000);
        draw_env.borrow_mut().draw_rect(&Vertex::new(offset, offset), 64, 64, &Color::aqua());
        if offset == 240 - 64 {
            offset = 0;
        }
        fb.swap();
    }
}
