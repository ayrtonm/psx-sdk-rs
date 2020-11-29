use core::mem::size_of;

use super::{Buffer, DoubleBuffer};
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
            const SIZE: usize = size_of::<Self>() / 4;
            pub fn from_double<const N: usize>(buffer: &mut DoubleBuffer<N>) -> [Self; 2] {
                let (buf0, buf1) = buffer.get(Self::SIZE);
                [$name::from_direct::<N>(buf0), $name::from_direct::<N>(buf1)]
            }
            pub fn from<const N: usize>(buffer: &mut Buffer<N>) -> Self {
                $name::from_direct::<N>(buffer.get(Self::SIZE))
            }
            fn from_direct<const N: usize>(buffer: &mut [u32]) -> Self {
                let ptr = buffer
                    .as_mut_ptr()
                    .cast::<Self>();
                let mut prim = unsafe { core::ptr::read(ptr) };
                prim.cmd = $cmd;
                prim.tag = (Self::SIZE as u32) << 24;
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
        }
    };
}

impl_PolyF!(3, PolyF3, 0x20);
impl_PolyF!(4, PolyF4, 0x28);
