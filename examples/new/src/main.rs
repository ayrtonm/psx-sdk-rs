#![no_std]
#![no_main]
#![feature(array_map)]

use core::cell::RefCell;
use psx::dma::{Step, BaseAddress, BlockControl, BlockSize, ChannelControl, Direction, SyncMode};
use psx::framebuffer::UncheckedFramebuffer;
use psx::gpu::color::Color;
use psx::gpu::primitives;
use psx::gpu::primitives::polyf::{PolyF3, PolyF4};
use psx::interrupt::IRQ;

psx::exe!();

fn main(mut mmio: MMIO) {
    let gp0 = RefCell::new(mmio.gp0);
    let gp1 = RefCell::new(mmio.gp1);
    mmio.dma_control.gpu(true);
    mmio.dma_control.otc(true);
    let mut fb = UncheckedFramebuffer::new((0, 0), (0, 240), (320, 240), &gp0, &gp1);

    let mut buffer = primitives::Buffer::<11>::new();

    let mut prim0 = PolyF3::new(&mut buffer, [(0, 0), (100, 0), (0, 100)], Color::BLUE);
    let mut prim1 = PolyF4::from(&mut buffer);
    prim1
        .vertices([(100, 100), (50, 100), (100, 50), (25, 25)])
        .color(Color::YELLOW);

    let mut ot = primitives::OT::<8>::new();

    mmio.otc_dma.base_address.set(ot.get(7));
    mmio.otc_dma.block_control.set(8);
    let clear_otc = mmio
        .otc_dma
        .channel_control
        .set_sync_mode(SyncMode::Immediate)
        .set_step(Step::Backward)
        .start(());
    clear_otc.wait();

    ot.add_prim(4, &mut prim1.tag).add_prim(4, &mut prim0.tag);

    gp1.borrow_mut().dma_direction(2);
    mmio.gpu_dma.base_address.set(ot.get(5));
    mmio.gpu_dma.block_control.set(BlockSize::LinkedList);
    let draw_prim = mmio
        .gpu_dma
        .channel_control
        .set_direction(Direction::FromMemory)
        .set_sync_mode(SyncMode::LinkedList)
        .start(());
    draw_prim.wait();

    loop {
        let c = prim0.color;
        prim0.color = prim1.color;
        prim1.color = c;

        mmio.gpu_dma.base_address.set(ot.get(5));
        mmio.gpu_dma.channel_control.start(()).wait();
        mmio.gpu_stat.sync();

        //mmio.int_stat.wait(IRQ::Vblank);
        mmio.int_stat.ack(IRQ::Vblank);

        fb.swap();
        mmio.gpu_stat.sync();
    }
}
