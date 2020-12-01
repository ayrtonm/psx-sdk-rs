#![no_std]
#![no_main]
#![feature(min_const_generics)]

use psx::framebuffer::Framebuffer;
use psx::gpu::color::Color;
use psx::gpu::primitive;
use psx::interrupt::IRQ;

psx::exe!();

fn main(mut mmio: MMIO) {
    mmio.dma_control.gpu(true).otc(true);
    let mut fb = Framebuffer::new((0, 0), (0, 240), (320, 240), &mut mmio.gp0, &mut mmio.gp1);

    let mut buffer = primitive::Buffer::<100>::new();
    let mut ot = primitive::OT::<8>::new();

    mmio.otc_dma.clear(&ot).wait();

    draw_scene2(&mut buffer, &mut ot);

    draw_scene(&mut buffer, &mut ot);
    mmio.gpu_dma.prepare_ot(&mut mmio.gp1).send(&ot).wait();

    loop {
        mmio.gpu_dma.send(&ot).wait();
        mmio.gpu_stat.sync();

        mmio.int_stat.ack(IRQ::Vblank);

        fb.swap(&mut mmio.gp0, &mut mmio.gp1);
        mmio.gpu_stat.sync();
    }
}

fn draw_scene<const N: usize, const M: usize>(
    buffer: &mut primitive::Buffer<N>, ot: &mut primitive::OT<M>,
) {
    let prim0 = buffer
        .PolyF3()
        .unwrap()
        .vertices([(0, 0), (100, 0), (0, 100)])
        .color(Color::BLUE);
    let prim1 = buffer
        .PolyF4()
        .unwrap()
        .vertices([(100, 100), (50, 100), (100, 50), (25, 25)])
        .color(Color::YELLOW);
    ot.add_prim(4, prim1).add_prim(4, prim0);
}

fn draw_scene2<const N: usize, const M: usize>(
    buffer: &mut primitive::Buffer<N>, ot: &mut primitive::OT<M>,
) {
    let prim0 = buffer
        .PolyF3()
        .unwrap()
        .vertices([(25, 25), (75, 0), (75, 100)])
        .color(Color::RED);
    ot.add_prim(4, prim0);
}
