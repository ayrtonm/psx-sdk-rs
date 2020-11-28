#![no_std]
#![no_main]
#![feature(min_const_generics)]

use core::cell::RefCell;
use psx::framebuffer::UncheckedFramebuffer;
use psx::gpu::color::Color;
use psx::gpu::primitive;
use psx::gpu::primitive::{PolyF3, PolyF4};

psx::exe!();

fn main(mut mmio: MMIO) {
    let gp0 = RefCell::new(mmio.gp0);
    let gp1 = RefCell::new(mmio.gp1);
    mmio.dma_control.gpu(true);
    mmio.dma_control.otc(true);
    let mut fb = UncheckedFramebuffer::new((0, 0), (0, 240), (320, 240), &gp0, &gp1);

    let mut buffer = primitive::Buffer::<11>::new();

    let prim0 = PolyF3::new(&mut buffer, [(0, 0), (100, 0), (0, 100)], Color::blue());

    let mut prim1 = PolyF4::from(&mut buffer);

    prim1
        .vertices([(100, 100), (50, 1000), (1000, 50), (50, 50)])
        .color(Color::yellow());
    prim1.as_mut().color = prim0.as_ref().color;

    loop {
        fb.swap();
    }
}
