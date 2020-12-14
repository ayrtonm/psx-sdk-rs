#![no_std]
#![no_main]
#![feature(min_const_generics, array_map, bool_to_option)]

extern crate alloc;
use alloc::vec;

use psx::bios;
use psx::framebuffer::UnsafeFramebuffer;
use psx::gpu::Color;
use psx::gte::GTE;
use psx::interrupt::IRQ;
use psx::mmio::gpu::GP1;
use psx::mmio::{dma, Enabled, MMIO};

mod draw;
mod map;
mod wall;
use crate::draw::draw;
use crate::map::Map;
pub use crate::wall::Wall;

pub struct IO {
    gp1: GP1,
    gpu_dma: dma::gpu::Channel<Enabled>,
    otc_dma: dma::otc::Channel<Enabled>,
}

#[no_mangle]
fn main(mut mmio: MMIO, mut gte: GTE) {
    // TODO: This breaks mednafen
    gte.enable();
    bios::init_heap(0x9F80_0000, 1024);
    let dma_control = &mut mmio.dma_control;
    let mut io = IO {
        gp1: mmio.gp1,
        gpu_dma: mmio.gpu_dma.enable(dma_control),
        otc_dma: mmio.otc_dma.enable(dma_control),
    };
    mmio.int_mask.enable(IRQ::Vblank);
    let map = Map::new(vec![
        Wall::new((-4, 2), (0, -2), Color::AQUA),
        Wall::new((0, -2), (4, -2), Color::MINT),
        Wall::new((4, -2), (8, 2), Color::INDIGO),
        Wall::new((-4, 2), (-8, 2), Color::ORANGE),
        Wall::new((-1, 0), (1, 0), Color::VIOLET), // 1
    ]);
    let partition = map.partition().unwrap();
    let traversal = partition.traverse();
    let mut fb = UnsafeFramebuffer::default();
    loop {
        draw(&traversal, &mut io);
        mmio.gpu_stat.sync();
        mmio.int_stat.ack(IRQ::Vblank);
        mmio.int_stat.wait(IRQ::Vblank);
        fb.swap();
    }
}
