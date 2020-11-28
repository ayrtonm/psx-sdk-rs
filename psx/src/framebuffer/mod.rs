//! Basic framebuffer routines.
//!
//! [`Framebuffer`] uses mutable references to [`GP0`] and [`GP1`] to initialize
//! and swap buffers. While this let's us apply the borrow checker rules to I/O
//! registers statically for compile-time errors, it is occasionally too
//! restrictive and not very ergonomic. [`UncheckedFramebuffer`] is
//! an alternative which stores references to [`RefCell`]s with [`GP0`] and
//! [`GP1`] to avoid having to pass in mutable references every time we swap
//! buffers. In exchange for this flexibility, we have to apply the borrow
//! checker rules dynamically and get run-time errors instead.

use crate::gpu::vertex::Vertex;
use crate::mmio::gpu::{GP0, GP1};
use core::cell::RefCell;

pub struct UncheckedFramebuffer<'a, 'b> {
    fb: Framebuffer,
    gp0: &'a RefCell<GP0>,
    gp1: &'b RefCell<GP1>,
}

impl<'a, 'b> UncheckedFramebuffer<'a, 'b> {
    pub fn new<T, U, V>(
        buf0: T, buf1: U, res: V, gp0: &'a RefCell<GP0>, gp1: &'b RefCell<GP1>,
    ) -> Self
    where Vertex: From<T> + From<U> + From<V> {
        UncheckedFramebuffer {
            fb: Framebuffer::new(
                buf0,
                buf1,
                res,
                &mut gp0.borrow_mut(),
                &mut gp1.borrow_mut(),
            ),
            gp0,
            gp1,
        }
    }

    pub fn swap(&mut self) {
        self.fb
            .swap(&mut self.gp0.borrow_mut(), &mut self.gp1.borrow_mut())
    }
}

enum Buffer {
    One,
    Two,
}

pub struct Framebuffer {
    display: Buffer,
    buffers: (Vertex, Vertex),
    _res: Vertex,
}

impl Framebuffer {
    pub fn new<T, U, V>(buf0: T, buf1: U, res: V, _gp0: &mut GP0, _gp1: &mut GP1) -> Self
    where Vertex: From<T> + From<U> + From<V> {
        Framebuffer {
            display: Buffer::One,
            buffers: (Vertex::from(buf0), Vertex::from(buf1)),
            _res: Vertex::from(res),
        }
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

    fn draw(&mut self, buffer: Buffer, _gp0: &mut GP0) {
        let _buffer = self.buffer(buffer);
    }

    fn display(&mut self, buffer: Buffer, _gp0: &mut GP0, _gp1: &mut GP1) {
        let _buffer = self.buffer(buffer);
    }
}
