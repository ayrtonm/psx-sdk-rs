#![no_std]
#![no_main]

use psx::gpu::color::Color;
use psx::gpu::framebuffer::Framebuffer;
use psx::gpu::vertex::{Pixel, Vertex};
use psx::gpu::{DrawPort, Hres, Vres};
use psx::interrupt::IRQ;

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
    let radius = 32;
    loop {
        draw_port.draw_shaded_quad(
            &Vertex::rect((160, 120), (320, 240)),
            &[Color::aqua(), Color::black(), Color::aqua(), Color::black()],
        );
        if pos.x() + radius >= 320 || pos.x() <= radius {
            vel = vel.map(|(x, y)| (-x, y));
        }
        if pos.y() + radius >= 240 || pos.y() <= radius {
            vel = vel.map(|(x, y)| (x, -y));
        }
        pos = pos.shift(&vel);
        draw_port.draw_circle(&pos, radius, &Color::orange());
        int_stat.ack_wait(IRQ::Vblank);
        fb.swap(&mut draw_port, &mut disp_port);
    }
}

trait DrawPrimitive {
    fn draw_circle(&mut self, center: &Vertex, radius: Pixel, color: &Color) -> &mut DrawPort;
}

// It'd be more efficient to serialize the pixel data once then send it
// repeatedly through the GPU DMA channel to free up the CPU for other tasks,
// but this is mostly to demo how to draw 2D primitive. This is a good reminder
// to not add this as is to the psx crate though.
impl DrawPrimitive for DrawPort {
    fn draw_circle(&mut self, center: &Vertex, radius: Pixel, color: &Color) -> &mut DrawPort {
        let radius = radius as i16;
        for i in 0..=2 * radius {
            for j in 0..=2 * radius {
                let a = i - radius;
                let b = j - radius;
                let rad_sq = ((a * a) + (b * b)) as f32;
                let circle = (radius * radius) as f32;
                if rad_sq <= circle {
                    self.draw_pixel(center.shift((a, b)), color);
                }
            }
        }
        self
    }
}
