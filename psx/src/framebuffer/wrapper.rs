use super::Framebuffer;
use crate::gpu::vertex::Vertex;
use crate::mmio::gpu::{GP0, GP1};
use core::cell::RefCell;

pub struct UnsafeFramebuffer {
    fb: Framebuffer,
    gp0: GP0,
    gp1: GP1,
}

impl UnsafeFramebuffer {
    pub fn new<T, U, V>(buf0: T, buf1: U, res: V) -> Self
    where Vertex: From<T> + From<U> + From<V> {
        let (mut gp0, mut gp1) = unsafe { (GP0::new(), GP1::new()) };
        let fb = Framebuffer::new(buf0, buf1, res, &mut gp0, &mut gp1);
        UnsafeFramebuffer { fb, gp0, gp1 }
    }

    pub fn swap(&mut self) {
        self.fb.swap(&mut self.gp0, &mut self.gp1)
    }
}

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
