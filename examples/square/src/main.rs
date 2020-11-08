#![no_std]
#![no_main]

libpsx::exe!();

use libpsx::gpu;
use libpsx::gpu::{Res, Hres, Vres};
use libpsx::gpu::framebuffer::Framebuffer;

use libpsx::gpu::color::{Palette, Color};
use libpsx::gpu::position::Position;
use libpsx::gpu::draw;
use libpsx::util::delay;

fn main() {
    let ctxt = gpu::Ctxt::new();
    let res = Res::new(Hres::H320, Vres::V240);
    let mut fb = Framebuffer::new(ctxt, (0, 0), (320, 0), res);

    let mut cols = [Color::red(), Color::green(), Color::blue(), Color::yellow()];
    let rect = Position::rect(Position::new(80, 60), 160, 120);
    loop {
        let pal = Palette::Shaded(cols);
        cols.rotate_right(1);
        draw::frame(&rect, &pal, None);
        fb.swap();
        delay(1000000);
    }
}
