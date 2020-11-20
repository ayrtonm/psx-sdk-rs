#![no_std]
#![no_main]
#![feature(array_map)]

use core::cell::RefCell;

use psx::gpu::framebuffer::Framebuffer;
use psx::gpu::{DispPort, DmaSource, DrawPort, Hres, Vres};
use psx::include_u32;

psx::exe!();

fn mk_framebuffer<'a, 'b>(
    draw_port: &'a RefCell<DrawPort>, disp_port: &'b RefCell<DispPort>,
) -> Framebuffer<'a, 'b> {
    let buf0 = (0, 0);
    let buf1 = (0, 240);
    let res = (Hres::H320, Vres::V240);
    disp_port.borrow_mut().reset_gpu();
    disp_port.borrow_mut().dma(DmaSource::CPU);
    Framebuffer::new(draw_port, disp_port, buf0, buf1, res)
}

fn main(mut io: IO) {
    let draw_port = RefCell::new(io.take_draw_port().expect("DrawPort has been taken"));
    let disp_port = RefCell::new(io.take_disp_port().expect("DispPort has been taken"));
    mk_framebuffer(&draw_port, &disp_port);
    let ferris = include_u32!("../ferris.tim");

    draw_port
        .borrow_mut()
        .rect_to_vram((0, 0), (256, 256), &ferris);
    loop {}
}
