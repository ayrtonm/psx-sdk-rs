use core::mem::size_of;

use super::Buffer;
use crate::gpu::color::Color;
use crate::gpu::vertex::Vertex;

#[repr(C)]
pub struct PolyF3 {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub vertices: [Vertex; 3],
}

#[repr(C)]
pub struct PolyF4 {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub vertices: [Vertex; 4],
}

macro_rules! impl_PolyF {
    ($n:expr, $name:ident, $cmd:expr) => {
        impl $name {
            pub fn from<const N: usize>(buffer: &mut Buffer<N>) -> Self {
                let ptr = buffer
                    .get(size_of::<Self>() / 4)
                    .as_mut_ptr()
                    .cast::<Self>();
                let mut prim = unsafe { core::ptr::read(ptr) };
                prim.cmd($cmd);
                prim.tag = (size_of::<Self>() as u32 / 4) << 24;
                prim
            }

            pub fn new<T, const N: usize>(
                buffer: &mut Buffer<N>, vertices: [T; $n], color: Color,
            ) -> Self
            where Vertex: From<T> {
                let mut prim = $name::from(buffer);
                prim.vertices(vertices).color(color);
                prim
            }

            pub fn vertices<T>(&mut self, vertices: [T; $n]) -> &mut Self
            where Vertex: From<T> {
                self.vertices = vertices.map(|t| Vertex::from(t));
                self
            }

            pub fn color(&mut self, color: Color) -> &mut Self {
                self.color = color;
                self
            }

            pub fn cmd(&mut self, cmd: u8) -> &mut Self {
                self.cmd = cmd;
                self
            }
        }
    };
}

impl_PolyF!(3, PolyF3, 0x20);
impl_PolyF!(4, PolyF4, 0x28);
