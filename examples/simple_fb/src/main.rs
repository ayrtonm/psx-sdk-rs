#![no_std]
#![no_main]

use core::cell::RefCell;

use libpsx::{IOCX, delay};
use libpsx::gpu::vertex::Vertex;
use libpsx::gpu::color::Color;
use libpsx::gpu::env::{Hres, Vres};
use libpsx::gpu::env::framebuffer::Framebuffer;

libpsx::exe!();

fn main() {
    let draw_env = unsafe {
        RefCell::new(IOCX.take_draw_env().unwrap())
    };
    let display_env = unsafe {
        RefCell::new(IOCX.take_display_env().unwrap())
    };
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
