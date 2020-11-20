#![no_std]
#![no_main]

use psx::delay;
use psx::gpu::color::Color;
use psx::gpu::framebuffer::Framebuffer;
use psx::gpu::vertex::{Pixel, Vertex};
use psx::gpu::{DrawPort, Hres, Vres};

psx::exe!();

fn main(mut io: IO) {
    let mut draw_port = io.take_draw_port().expect("DrawPort has been taken");
    let mut disp_port = io.take_disp_port().expect("DisplaEnv has been taken");
    let res = (Hres::H320, Vres::V240);
    let buf0 = (0, 0);
    let buf1 = (0, 240);
    let mut fb = Framebuffer::new(&mut draw_port, &mut disp_port, buf0, buf1, res);
    let mut offset = 0;
    let center = Vertex::new(200, 100);
    loop {
        offset += 1;
        delay(100000);
        draw_port.draw_square((offset, offset), 64, &Color::aqua())
            .draw_circle(&center, 32, &Color::orange())
            .draw_circle(&center.shift(32, 32), 24, &Color::indigo())
            .draw_circle(&center.shift(-32, 64), 32, &Color::mint())
            .draw_circle(&center.shift(122, 54), 42, &Color::green())
            .draw_circle(&center.shift(-120, 54), 42, &Color::red());
        if offset == 240 - 64 {
            offset = 0;
        }
        fb.swap(&mut draw_port, &mut disp_port);
    }
}

trait DrawPrimitive {
    fn draw_circle(&mut self, center: &Vertex, radius: Pixel, color: &Color) -> &mut DrawPort;
}

// It'd be more efficient to serialize the pixel data once then send it repeatedly through the GPU
// DMA channel to free up the CPU for other tasks, but this is mostly to demo how to draw 2D
// primitive. This is a good reminder to not add this as is to the psx crate though.
impl DrawPrimitive for DrawPort {
    fn draw_circle(&mut self, center: &Vertex, radius: Pixel, color: &Color) -> &mut DrawPort {
        let radius = radius as i16;
        for i in 0..=2 * radius {
            for j in 0..=2 * radius {
                let a = i - radius;
                let b = j - radius;
                let rad_sq = (a * a + b * b) as f32;
                let circle = (radius * radius) as f32;
                if rad_sq <= circle {
                    self.draw_pixel(center.shift(a, b), color);
                }
            }
        }
        self
    }
}
