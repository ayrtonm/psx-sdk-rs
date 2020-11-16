#![no_std]
#![no_main]

use core::cell::RefCell;

use libpsx::delay;
use libpsx::gpu::color::Color;
use libpsx::gpu::framebuffer::Framebuffer;
use libpsx::gpu::vertex::Vertex;
use libpsx::gpu::{Hres, Vres};

libpsx::exe!();

fn main(mut ctxt: Ctxt) {
    let draw_port = RefCell::new(ctxt.take_draw_port().expect("DrawPort has been taken"));
    let disp_port = RefCell::new(ctxt.take_disp_port().expect("DisplaEnv has been taken"));
    let res = (Hres::H320, Vres::V240);
    let buf0 = (0, 0);
    let buf1 = (0, 240);
    let mut fb = Framebuffer::new(&draw_port, &disp_port, buf0, buf1, res);
    let mut offset = 0;
    loop {
        offset += 1;
        delay(100000);
        draw_port
            .borrow_mut()
            .draw_rect(&Vertex::new(offset, offset), 64, 64, &Color::aqua());
        if offset == 240 - 64 {
            offset = 0;
        }
        fb.swap();
    }
}
