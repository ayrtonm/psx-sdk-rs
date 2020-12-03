#![no_std]
#![no_main]
#![feature(array_map, min_const_generics)]

use psx::{unzip_now, include_u32};
use psx::tim::TIM;
use psx::framebuffer::Framebuffer;
use psx::gpu::color::Color;
use psx::gpu::vertex::Vertex;
use psx::gpu::primitive;
use psx::gpu::primitive::polyft::PolyFT4;
use psx::gpu::primitive::polyf::PolyF4;
use psx::interrupt::IRQ;

psx::exe!();

fn main(mut mmio: MMIO) {
    mmio.dma_control.gpu(true).otc(true);
    let mut fb = Framebuffer::new((0, 0), (0, 240), (320, 240), &mut mmio.gp0, &mut mmio.gp1);

    let mut ferris = include_u32!("../ferris.tim");
    let tim = TIM::new(&mut ferris);
    mmio.gp1.dma_direction(2);
    let (tpage, clut) = mmio.gpu_dma.load_tim(&tim);
    let polyft4 = PolyFT4 {
        color: Color::WHITE,
        cmd: 0x2C,
        v0: (0, 0).into(),
        t0: (0, 0).into(),
        clut: clut.into(),
        v1: (320, 0).into(),
        t1: (255, 0).into(),
        tpage,
        v2: (0, 240).into(),
        t2: (0, 255).into(),
        _pad0: 0,
        v3: (320, 240).into(),
        t3: (255, 255).into(),
        _pad1: 0,
    };
    mmio.gp0.send(&polyft4);
    fb.swap(&mut mmio.gp0, &mut mmio.gp1);
    loop {}

    let buffer = primitive::Buffer::<100>::new();
    let mut ot = primitive::OT::<8>::new();

    mmio.otc_dma.clear(&ot).wait();

    draw_scene2(&buffer, &mut ot);

    draw_scene(&buffer, &mut ot);
    ot.add_prim(4, &mut polyft4);
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
    buffer: &primitive::Buffer<N>, ot: &mut primitive::OT<M>,
) {
    #[repr(C)]
    struct Composite {
        pub a: primitive::polyf::PolyF3,
        pub b: primitive::polyf::PolyF4,
    }
    impl primitive::Init for Composite {
        fn init(&mut self) {
            self.a.cmd();
            self.b.cmd();
        }
    }
    let composite = buffer.alloc::<Composite>().unwrap();
    composite
        .packet()
        .a
        .vertices([(0, 0), (100, 0), (0, 100)])
        .color(Color::BLUE);
    composite
        .packet()
        .b
        .vertices([(100, 100), (50, 100), (100, 50), (25, 25)])
        .color(Color::YELLOW);
    ot.add_prim(4, composite);
}

fn draw_scene2<const N: usize, const M: usize>(
    buffer: &primitive::Buffer<N>, ot: &mut primitive::OT<M>,
) {
    let prim0 = buffer
        .PolyF3()
        .unwrap()
        .vertices([(25, 25), (75, 0), (75, 100)])
        .color(Color::RED);
    let buffer2 = primitive::Buffer::<20>::new();
    let prim1 = buffer2
        .PolyF4()
        .unwrap()
        .vertices([(200, 130), (200, 260), (275, 230), (275, 260)])
        .color(Color::ORANGE);
    // Uncommenting the latter part should not produce a valid program
    // Review Embedonomicon DMA chapter to try and fix this
    ot.add_prim(4, prim0); //.add_prim(4, prim1);
}
