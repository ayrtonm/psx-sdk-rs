#![no_std]
#![no_main]

use psx::delay;
use psx::gpu::color::Color;
use psx::gpu::framebuffer::Framebuffer;
use psx::gpu::{Hres, Vres};

psx::exe!();

fn main(mut io: IO) {
    let mut draw_port = io.take_draw_port().expect("DrawPort has been taken");
    let mut disp_port = io.take_disp_port().expect("DisplaEnv has been taken");
    let res = (Hres::H320, Vres::V240);
    let buf0 = (0, 0);
    let buf1 = (0, 240);
    let mut fb = Framebuffer::new(&mut draw_port, &mut disp_port, buf0, buf1, res);
    let mut offset = 0;
    loop {
        offset += 1;
        delay(100000);
        draw_port.draw_square((offset, offset), 64, &Color::aqua());
        if offset == 240 - 64 {
            offset = 0;
        }
        fb.swap(&mut draw_port, &mut disp_port);
    }
}
