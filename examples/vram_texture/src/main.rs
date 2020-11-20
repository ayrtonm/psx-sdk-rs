#![no_std]
#![no_main]
#![feature(array_map)]

use psx::gpu::framebuffer::Framebuffer;
use psx::gpu::{DispPort, DmaSource, DrawPort, Hres, Vres};
use psx::include_u32;

psx::exe!();

fn mk_framebuffer(draw_port: &mut DrawPort, disp_port: &mut DispPort) -> Framebuffer {
    let buf0 = (0, 0);
    let buf1 = (0, 240);
    let res = (Hres::H320, Vres::V240);
    disp_port.reset_gpu();
    disp_port.dma(DmaSource::CPU);
    Framebuffer::new(draw_port, disp_port, buf0, buf1, res)
}

fn main(mut io: IO) {
    let mut draw_port = io.take_draw_port().expect("DrawPort has been taken");
    let mut disp_port = io.take_disp_port().expect("DispPort has been taken");
    mk_framebuffer(&mut draw_port, &mut disp_port);
    let ferris = include_u32!("../ferris.tim");

    draw_port.rect_to_vram((0, 0), (256, 256), &ferris);
    loop {}
}
