#![no_std]
#![no_main]
#![feature(array_map, once_cell)]

use psx::gpu::color::Color;
use psx::gpu::framebuffer::Framebuffer;
use psx::gpu::primitives::{shaded_quad, textured_quad};
use psx::gpu::vertex::Vertex;
use psx::interrupt::IRQ;
use psx::*;

psx::exe!();

fn main(mut io: IO) {
    let mut draw_port = io.take_draw_port().expect("DrawPort has been taken");
    let mut disp_port = io.take_disp_port().expect("DispPort has been taken");
    let mut int_stat = io.take_int_stat().expect("interrupt::Stat has been taken");
    let mut gpu_dma = io.take_gpu_dma().expect("dma::Gpu has been taken");
    let mut fb = Framebuffer::new(
        &mut draw_port,
        &mut disp_port,
        Vertex::zero(),
        (0, 240),
        (320, 240),
        None,
    );
    let mut pos = Vertex::new(200, 100);
    let mut vel = Vertex::new(4, 2);
    let ferris = unzip!("../ferris-8bpp.tim.zip");
    let tim = tim!(ferris);
    let (page, clut) = tim.load(&mut draw_port, &mut gpu_dma);
    let bg = shaded_quad(
        Vertex::offset_rect(Vertex::zero(), (320, 240)),
        [Color::aqua(), Color::black(), Color::aqua(), Color::black()],
    );
    let size = 64;
    let half_size = 32;
    let mut fg = textured_quad(
        Vertex::square(pos, size),
        Color::white(),
        [(0, 0), (0, 255), (255, 0), (255, 255)],
        page,
        clut,
    );
    loop {
        if pos.x() + half_size >= 320 || pos.x() <= half_size {
            vel.apply(|x, y| (-x, y));
        }
        if pos.y() + half_size >= 240 || pos.y() <= half_size {
            vel.apply(|x, y| (x, -y));
        }
        pos += vel;
        fg.map(|v, _, _| {
            *v = Vertex::square(pos, size).map(|v| (v.x(), v.y() + (v.x() - pos.x())).into())
        });
        draw_port.send(&bg).send(&fg);
        int_stat.ack_wait(IRQ::Vblank);
        fb.swap(&mut draw_port, &mut disp_port);
    }
}
