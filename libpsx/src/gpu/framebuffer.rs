use core::cell::RefCell;
use crate::gpu::{DrawEnv, DisplayEnv};
use crate::gpu::res::{Component, Res, Vmode, Depth};
use crate::gpu::vertex::{Length, Vertex};
use crate::gpu::color::Color;

type BufferLocation = (Component, Component);

enum Buffer { One, Two }

pub struct Framebuffer<'a, 'b> {
    draw_env: &'a RefCell<DrawEnv>,
    display_env: &'b RefCell<DisplayEnv>,
    display: Buffer,
    buffers: (BufferLocation, BufferLocation),
    res: Res,
}

impl<'a, 'b> Framebuffer<'a, 'b> {
    pub fn new(draw_env: &'a RefCell<DrawEnv>, display_env: &'b RefCell<DisplayEnv>, one: BufferLocation, two: BufferLocation, res: Res) -> Self {
        display_env.borrow_mut().horizontal(0, u32::from(&res.0));
        display_env.borrow_mut().vertical(0, u32::from(&res.1));
        display_env.borrow_mut().mode(&res.0, &res.1, Vmode::NTSC, Depth::Lo, false);
        let mut fb = Framebuffer {
            draw_env,
            display_env,
            display: Buffer::One,
            buffers: (one, two),
            res,
        };
        fb.draw(Buffer::Two);
        fb.display(Buffer::One);
        display_env.borrow_mut().on();
        fb
    }
    pub fn swap(&mut self) {
        match self.display {
            Buffer::One => {
                self.display = Buffer::Two;
                self.draw(Buffer::One);
                self.display(Buffer::Two);
            },
            Buffer::Two => {
                self.display = Buffer::One;
                self.draw(Buffer::Two);
                self.display(Buffer::One);
            },
        }
    }
    fn buffer_data(&self, buffer: Buffer) -> BufferLocation {
        match buffer {
            Buffer::One => self.buffers.0,
            Buffer::Two => self.buffers.1,
        }
    }
    fn draw(&mut self, buffer: Buffer) {
        let buffer = self.buffer_data(buffer);
        let hres = u32::from(&self.res.0);
        let vres = u32::from(&self.res.1);
        self.draw_env.borrow_mut().start(buffer.0, buffer.1);
        self.draw_env.borrow_mut().end(buffer.0 + hres, buffer.1 + vres);
        self.draw_env.borrow_mut().offset(buffer.0, buffer.1);
    }
    fn display(&mut self, buffer: Buffer) {
        let buffer = self.buffer_data(buffer);
        let hres = u32::from(&self.res.0);
        let vres = u32::from(&self.res.1);
        self.display_env.borrow_mut().start(buffer.0, buffer.1);
        self.draw_env.borrow_mut().draw_rect(&Vertex::zero(), hres as Length, vres as Length, &Color::black());
    }
}
