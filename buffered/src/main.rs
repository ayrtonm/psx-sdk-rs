#![no_std]
#![no_main]
#![feature(array_map)]

use libpsx::gpu::{Ctxt, Res, Hres, Vres};
use libpsx::gpu::framebuffer::Framebuffer;

use libpsx::gpu::color::{Color, Palette};
use libpsx::gpu::polygon::draw_quad;
use libpsx::gpu::position::Position;

use libpsx::util::delay;

libpsx::exe!();

fn main() {
    let ctxt = Ctxt::new();
    let res = Res { h: Hres::H320, v: Vres::V240 };
    let mut fb = Framebuffer::new(ctxt, (0, 0), (320, 0), res);

    let colors = [Color::indigo(), Color::aqua(), Color::orange(), Color::mint()];
    let pal = Palette::Shaded(colors);
    let quad = Position::rectangle(Position::zero(), 64, 64);
    let mut offset = 0;
    loop {
        offset += 1;
        if offset == 180 { offset = 0 };
        draw_quad(&quad.map(|p| Position::new(p.x() + offset, p.y() + offset)), &pal, None);
        fb.swap();
        delay(100000);
    }
}
