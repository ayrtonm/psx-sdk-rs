#![no_std]
#![no_main]
#![feature(array_map)]

use libpsx::gpu;
use libpsx::gpu::{Res, Hres, Vres};
use libpsx::gpu::framebuffer::Framebuffer;

use libpsx::gpu::color::{Color, Palette};
use libpsx::gpu::draw;
use libpsx::gpu::position::Position;

use libpsx::util::delay;

libpsx::exe!();

fn main() {
    let ctxt = gpu::Ctxt::new();
    let res = Res::new(Hres::H320, Vres::V240);
    let mut fb = Framebuffer::new(ctxt, (0, 0), (320, 0), res);

    let colors = [Color::indigo(), Color::aqua(), Color::orange(), Color::mint()];
    let pal = Palette::Shaded(colors);
    let quad = Position::rect(Position::zero(), 64, 64);
    let mut offset = 0;
    loop {
        offset += 1;
        if offset == 180 { offset = 0 };
        draw::quad(&quad.map(|p| Position::new(p.x() + offset, p.y() + offset)), &pal, None);
        fb.swap();
        delay(100000);
    }
}
