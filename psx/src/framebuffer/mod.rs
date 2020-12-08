//! Basic framebuffer routines.
//!
//! [`Framebuffer`] uses mutable references to [`GP0`] and [`GP1`] to initialize
//! and swap buffers. While this let's us apply the borrow checker rules to I/O
//! registers statically for compile-time errors, it is occasionally too
//! restrictive and not very ergonomic. [`UncheckedFramebuffer`] is
//! an alternative which stores references to [`RefCell`](core::cell::RefCell)s
//! with [`GP0`] and [`GP1`] to avoid having to pass in mutable references every
//! time we swap buffers. In exchange for this flexibility, we have to apply the
//! borrow checker rules dynamically and get run-time errors instead.

use crate::gpu::graphics::primitive::Tile;
use crate::gpu::Color;
use crate::gpu::Vertex;
use crate::gpu::{Depth, Vmode};
use crate::mmio::gpu::{GP0, GP1};

mod wrapper;

pub use wrapper::UncheckedFramebuffer;
pub use wrapper::UnsafeFramebuffer;

enum Buffer {
    One,
    Two,
}

pub struct Framebuffer {
    display: Buffer,
    buffers: (Vertex, Vertex),
    res: Vertex,
    clear: Color,
}

impl Framebuffer {
    pub fn new<T, U, V>(buf0: T, buf1: U, res: V, gp0: &mut GP0, gp1: &mut GP1) -> Self
    where Vertex: From<T> + From<U> + From<V> {
        let mut fb = Framebuffer {
            display: Buffer::One,
            buffers: (Vertex::from(buf0), Vertex::from(buf1)),
            res: Vertex::from(res),
            clear: Color::BLACK,
        };
        // Magic constants from nocash specs. If I remember correctly some PS1 games
        // made these adjustable in-game
        let hoffset = 0x248;
        let vmid = 0x88;
        gp1.horizontal(hoffset, hoffset + (fb.res.x() * 8))
            .vertical(vmid - (fb.res.y() / 2), vmid + (fb.res.y() / 2))
            .mode(fb.res.x(), fb.res.y(), Vmode::NTSC, Depth::Lo, false);
        fb.draw(Buffer::Two, gp0);
        fb.display(Buffer::One, gp0, gp1);
        gp1.on();
        fb
    }

    pub fn swap(&mut self, gp0: &mut GP0, gp1: &mut GP1) {
        match self.display {
            Buffer::One => {
                self.display = Buffer::Two;
                self.draw(Buffer::One, gp0);
                self.display(Buffer::Two, gp0, gp1);
            },
            Buffer::Two => {
                self.display = Buffer::One;
                self.draw(Buffer::Two, gp0);
                self.display(Buffer::One, gp0, gp1);
            },
        }
    }

    fn buffer(&self, buffer: Buffer) -> Vertex {
        match buffer {
            Buffer::One => self.buffers.0,
            Buffer::Two => self.buffers.1,
        }
    }

    fn draw(&mut self, buffer: Buffer, gp0: &mut GP0) {
        let buffer = self.buffer(buffer);
        gp0.start(buffer).end(buffer.shift(self.res)).offset(buffer);
    }

    fn display(&mut self, buffer: Buffer, gp0: &mut GP0, gp1: &mut GP1) {
        let buffer = self.buffer(buffer);
        gp1.start(buffer);
        let clear_screen = Tile {
            color: self.clear,
            cmd: 0x60,
            offset: 0.into(),
            size: self.res,
        };
        gp0.send(&clear_screen);
    }
}
