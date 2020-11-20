use crate::gpu::color::Color;
use crate::gpu::vertex::{Pixel, Vertex};
use crate::gpu::{Depth, DispPort, DrawPort, Res, Vmode};

type BufferLocation = (Pixel, Pixel);

enum Buffer {
    One,
    Two,
}

pub struct Framebuffer {
    display: Buffer,
    buffers: (BufferLocation, BufferLocation),
    res: Res,
}

impl Framebuffer {
    pub fn new(
        draw_port: &mut DrawPort, disp_port: &mut DispPort, one: BufferLocation,
        two: BufferLocation, res: Res,
    ) -> Self
    {
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

    fn buffer_data(&self, buffer: Buffer) -> BufferLocation {
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
            .end((buffer.0 + hres, buffer.1 + vres))
            .offset(buffer);
    }

    fn display(&mut self, draw_port: &mut DrawPort, disp_port: &mut DispPort, buffer: Buffer) {
        let buffer = self.buffer_data(buffer);
        let hres = (&self.res.0).into();
        let vres = (&self.res.1).into();
        disp_port.start(buffer);
        draw_port.draw_rect(Vertex::zero(), (hres, vres), &Color::black());
    }
}
