#![no_std]
#![no_main]
#![feature(array_map)]

use core::cell::RefCell;
use core::convert::TryInto;

use libpsx::gpu::framebuffer::Framebuffer;
use libpsx::gpu::{DispPort, DmaSource, DrawPort, Hres, Vres};

libpsx::exe!();

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

fn main(mut ctxt: Ctxt) {
    let draw_port = RefCell::new(ctxt.take_draw_port().expect("DrawPort has been taken"));
    let disp_port = RefCell::new(ctxt.take_disp_port().expect("DispPort has been taken"));
    mk_framebuffer(&draw_port, &disp_port);
    let ferris = include_bytes!("../ferris.tim");
    let mut ferris = ferris[0x14..]
        .chunks(4)
        .map(|c| u32::from_le_bytes(c.try_into().unwrap()));

    draw_port
        .borrow_mut()
        .rect_to_vram((0, 0), (256, 256), &mut ferris);
    loop {}
}
