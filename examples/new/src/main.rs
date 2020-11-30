#![no_std]
#![no_main]
#![feature(min_const_generics)]

use psx::framebuffer::Framebuffer;
use psx::gpu::color::Color;
use psx::gpu::primitive;
use psx::gpu::primitive::polyf::{PolyF3, PolyF4};
use psx::interrupt::IRQ;

psx::exe!();

fn main(mut mmio: MMIO) {
    mmio.dma_control.gpu(true);
    mmio.dma_control.otc(true);
    let mut fb = Framebuffer::new((0, 0), (0, 240), (320, 240), &mut mmio.gp0, &mut mmio.gp1);

    let mut buffer = primitive::Buffer::<11>::new();
    let mut ot = primitive::OT::<8>::new();

    mmio.otc_dma.clear(&ot).wait();

    inner(&mut buffer, &mut ot);

    mmio.gpu_dma
        .prepare_ot(&mut mmio.gp1)
        .send(&ot)
        .wait();

    loop {
        mmio.gpu_dma.send(&ot).wait();
        mmio.gpu_stat.sync();

        mmio.int_stat.ack(IRQ::Vblank);

        fb.swap(&mut mmio.gp0, &mut mmio.gp1);
        mmio.gpu_stat.sync();
    }
}

fn inner<const N: usize, const M: usize>(
    buffer: &mut primitive::Buffer<N>, ot: &mut primitive::OT<M>,
) {
    let prim0 = buffer
        .alloc::<PolyF3>()
        .as_mut()
        .vertices([(0, 0), (100, 0), (0, 100)])
        .color(Color::BLUE)
        .packet();
    let prim1 = buffer
        .alloc::<PolyF4>()
        .as_mut()
        .vertices([(100, 100), (50, 100), (100, 50), (25, 25)])
        .color(Color::YELLOW)
        .packet();
    ot.add_prim(4, prim1).add_prim(4, prim0);
}
