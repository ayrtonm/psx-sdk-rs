#![no_std]
#![no_main]
#![feature(once_cell)]

use psx::gpu::color::Color;
use psx::gpu::framebuffer::Framebuffer;
use psx::gpu::primitives::{point, shaded_quad, textured_quad};
use psx::gpu::vertex::{Pixel, Vertex};
use psx::gpu::Packet;
use psx::gpu::{DrawPort, Hres, Vres};
use psx::interrupt::IRQ;
use psx::*;

psx::exe!();

fn main(mut io: IO) {
    let mut draw_port = io.take_draw_port().expect("DrawPort has been taken");
    let mut disp_port = io.take_disp_port().expect("DispPort has been taken");
    let mut int_stat = io.take_int_stat().expect("interrupt::Stat has been taken");
    let res = (Hres::H320, Vres::V240);
    let buf0 = (0, 0);
    let buf1 = (0, 240);
    let mut fb = Framebuffer::new(&mut draw_port, &mut disp_port, buf0, buf1, res);
    let mut pos = Vertex::new(200, 100);
    let mut vel = Vertex::new(4, 2);
    let ferris = unzip!("../ferris.tim.zip");
    let tim = tim!(ferris);
    draw_port.to_vram((320, 0), (256, 256), tim.bitmap().body());
    let bg = shaded_quad(
        Vertex::offset_rect(Vertex::zero(), (320, 240)),
        [Color::aqua(), Color::black(), Color::aqua(), Color::black()],
    );
    let palette = [
        Color::indigo(),
        Color::orange(),
        Color::mint(),
        Color::aqua(),
    ];
    let size = 32;
    let half_size = 16;
    let mut fg = textured_quad(
        Vertex::square(pos, size),
        Color::indigo(),
        [(0, 0), (0, 255), (255, 0), (255, 255)],
        0,
        (5, 0),
    );
    loop {
        if pos.x() + half_size >= 320 || pos.x() <= half_size {
            vel.apply(|x, y| (-x, y));
        }
        if pos.y() + half_size >= 240 || pos.y() <= half_size {
            vel.apply(|x, y| (x, -y));
        }
        pos += vel;
        fg.map(|v, _, _| *v = Vertex::square(pos, size));
        draw_port.send(&bg).send(&fg);
        int_stat.ack_wait(IRQ::Vblank);
        fb.swap(&mut draw_port, &mut disp_port);
    }
}
