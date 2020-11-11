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
    let gp0 = unsafe {
        RefCell::new(IOCX.take_gp0().unwrap())
    };
    let gp1 = unsafe {
        RefCell::new(IOCX.take_gp1().unwrap())
    };
    let res = (Hres::H320, Vres::V240);
    let buf0 = (0, 0);
    let buf1 = (0, 240);
    let mut fb = Framebuffer::new(&gp0, &gp1, buf0, buf1, res);
    let mut offset = 0;
    loop {
        offset += 1;
        delay(100000);
        gp0.borrow_mut().draw_rect(&Vertex::new(offset, offset), 64, 64, &Color::aqua());
        if offset == 240 - 64 {
            offset = 0;
        }
        fb.swap();
    }
}
