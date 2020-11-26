use crate::gpu::color::Color;
use crate::gpu::primitives::rectangle;
use crate::gpu::vertex::Vertex;
use crate::gpu::{Depth, DispPort, DrawPort, Vmode};

enum Buffer {
    One,
    Two,
}

pub struct Attributes {
    pub mode: Vmode,
    pub color_depth: Depth,
    pub interlacing: bool,
}

pub struct Framebuffer {
    display: Buffer,
    buffers: (Vertex, Vertex),
    res: Vertex,
}

impl Framebuffer {
    pub fn new<T, U, V>(
        draw_port: &mut DrawPort, disp_port: &mut DispPort, one: T, two: U, res: V,
        attr: Option<Attributes>,
    ) -> Self
    where
        Vertex: From<T> + From<U> + From<V>,
    {
        let attr = attr.unwrap_or(Attributes {
            mode: Vmode::NTSC,
            color_depth: Depth::Lo,
            interlacing: false,
        });
        let mut fb = Framebuffer {
            display: Buffer::One,
            buffers: (Vertex::from(one), Vertex::from(two)),
            res: Vertex::from(res),
        };
        let hoffset = 0x248;
        let vmid = 0x88;
        disp_port
            .horizontal(hoffset, hoffset + (fb.res.x() * 8))
            .vertical(vmid - (fb.res.y() / 2), vmid + (fb.res.y() / 2))
            .mode(
                fb.res.x(),
                fb.res.y(),
                attr.mode,
                attr.color_depth,
                attr.interlacing,
            );
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
        draw_port
            .start(buffer)
            .end(buffer.shift(self.res))
            .offset(buffer);
    }

    fn display(&mut self, draw_port: &mut DrawPort, disp_port: &mut DispPort, buffer: Buffer) {
        let buffer = self.buffer_data(buffer);
        disp_port.start(buffer);
        let clear_screen = rectangle(Vertex::zero(), self.res, Color::black());
        draw_port.send(&clear_screen);
    }
}
