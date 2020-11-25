use crate::gpu::color::Color;
use crate::gpu::primitives::rectangle;
use crate::gpu::vertex::{Pixel, Vertex};
use crate::gpu::{Depth, DispPort, DrawPort, Res, Vmode};

enum Buffer {
    One,
    Two,
}

pub struct Framebuffer {
    display: Buffer,
    buffers: (Vertex, Vertex),
    res: Res,
}

impl Framebuffer {
    pub fn new<T, U>(
        draw_port: &mut DrawPort, disp_port: &mut DispPort, one: T, two: U, res: Res,
    ) -> Self
    where Vertex: From<T> + From<U> {
        let one = Vertex::from(one);
        let two = Vertex::from(two);
        disp_port
            .horizontal(0, (&res.0).into())
            .vertical(0, (&res.1).into())
            .mode(&res.0, &res.1, Vmode::NTSC, Depth::Lo, false);
        let mut fb = Framebuffer {
            display: Buffer::One,
            buffers: (one, two),
            res,
        };
        fb.draw(draw_port, Buffer::Two);
        fb.display(draw_port, disp_port, Buffer::One);
        disp_port.on();
        fb
    }

    pub fn swap(&mut self, draw_port: &mut DrawPort, disp_port: &mut DispPort) {
        match self.display {
            Buffer::One => {
                self.display = Buffer::Two;
                self.draw(draw_port, Buffer::One);
                self.display(draw_port, disp_port, Buffer::Two);
            },
            Buffer::Two => {
                self.display = Buffer::One;
                self.draw(draw_port, Buffer::Two);
                self.display(draw_port, disp_port, Buffer::One);
            },
        }
    }

    fn buffer_data(&self, buffer: Buffer) -> Vertex {
        match buffer {
            Buffer::One => self.buffers.0,
            Buffer::Two => self.buffers.1,
        }
    }

    fn draw(&mut self, draw_port: &mut DrawPort, buffer: Buffer) {
        let buffer = self.buffer_data(buffer);
        let hres: Pixel = (&self.res.0).into();
        let vres: Pixel = (&self.res.1).into();
        draw_port
            .start(buffer)
            .end(buffer.shift((hres, vres)))
            .offset(buffer);
    }

    fn display(&mut self, draw_port: &mut DrawPort, disp_port: &mut DispPort, buffer: Buffer) {
        let buffer = self.buffer_data(buffer);
        let hres = (&self.res.0).into();
        let vres = (&self.res.1).into();
        disp_port.start(buffer);
        let clear_screen = rectangle(Vertex::zero(), (hres, vres), Color::black());
        draw_port.send(&clear_screen);
    }
}
