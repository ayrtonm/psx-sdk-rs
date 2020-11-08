use crate::gpu::{Ctxt, Res};
use crate::gpu::polygon::draw_rect;
use crate::gpu::color::Color;
use crate::gpu::position::Position;

type BufferData = (u32, u32);

struct ResData {
    h: u32,
    v: u32,
}

enum Buffer {
    One,
    Two,
}

pub struct Framebuffer {
    ctxt: Ctxt,
    display: Buffer,
    buffers: (BufferData, BufferData),
    res: ResData,
}

impl Framebuffer {
    pub fn new(ctxt: Ctxt, buffer_one: BufferData, buffer_two: BufferData, res: Res) -> Self {
        let res = ResData { h: res.h.into(), v: res.v.into() };
        let fb = Framebuffer {
            ctxt,
            display: Buffer::One,
            buffers: (buffer_one, buffer_two),
            res,
        };
        fb.draw(buffer_two);
        fb.display(buffer_one);
        fb.init();
        fb
    }
    fn init(&self) {
        self.ctxt.display_env
            .horizontal(0, self.res.h)
            .vertical(0, self.res.v)
            .on();
    }
    pub fn swap(&mut self) {
        match self.display {
            Buffer::One => {
                self.display = Buffer::Two;
                self.display(self.buffer_data(Buffer::Two));
                self.draw(self.buffer_data(Buffer::One));
            },
            Buffer::Two => {
                self.display = Buffer::One;
                self.display(self.buffer_data(Buffer::One));
                self.draw(self.buffer_data(Buffer::Two));
            },
        }
    }
    fn buffer_data(&self, buffer: Buffer) -> BufferData {
        match buffer {
            Buffer::One => self.buffers.0,
            Buffer::Two => self.buffers.1,
        }
    }
    fn display(&self, buffer_data: BufferData) {
        self.ctxt
            .display_env
            .start(buffer_data.0, buffer_data.1);
    }
    fn draw(&self, buffer_data: BufferData) {
        let hres = self.res.h;
        let vres = self.res.v;
        self.ctxt
            .draw_env
            .start(buffer_data.0, buffer_data.1)
            .end(buffer_data.0 + hres, buffer_data.1 + vres)
            .offset(buffer_data.0, buffer_data.1);
        draw_rect(&Position::zero(), hres as u16, vres as u16, &Color::black(), None);
    }
}
