#![no_std]
#![no_main]

use core::cell::RefCell;

use psx::delay;
use psx::gpu::color::Color;
use psx::gpu::framebuffer::Framebuffer;
use psx::gpu::vertex::Vertex;
use psx::gpu::{Hres, Vres};

psx::exe!();

fn main(mut io: IO) {
    let draw_port = RefCell::new(io.take_draw_port().expect("DrawPort has been taken"));
    let disp_port = RefCell::new(io.take_disp_port().expect("DisplaEnv has been taken"));
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
