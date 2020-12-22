use super::Framebuffer;
use crate::gpu::Vertex;
use crate::mmio::dma;
use crate::mmio::gpu::{GP0, GP1};
use core::cell::RefCell;

impl Default for UnsafeFramebuffer {
    fn default() -> Self {
        UnsafeFramebuffer::new(0, (0, 240), (320, 240))
    }
}

pub struct UnsafeFramebuffer {
    fb: Framebuffer,
    gp0: GP0,
    gp1: GP1,
    gpu_dma: dma::gpu::Channel,
}

impl UnsafeFramebuffer {
    pub fn new<T, U, V>(buf0: T, buf1: U, res: V) -> Self
    where Vertex: From<T> + From<U> + From<V> {
        unsafe {
            let mut gp0 = GP0::new();
            let mut gp1 = GP1::new();
            let mut gpu_dma = dma::gpu::Channel::new();
            let fb = Framebuffer::new(buf0, buf1, res, &mut gp0, &mut gp1, &mut gpu_dma);
            UnsafeFramebuffer {
                fb,
                gp0,
                gp1,
                gpu_dma,
            }
        }
    }

    pub fn swap(&mut self) {
        self.fb
            .swap(&mut self.gp0, &mut self.gp1, &mut self.gpu_dma)
    }
}

pub struct UncheckedFramebuffer<'a, 'b> {
    fb: Framebuffer,
    gp0: &'a RefCell<GP0>,
    gp1: &'b RefCell<GP1>,
}
/*

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
*/
