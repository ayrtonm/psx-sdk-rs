use super::{Allocatable, Packet};
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
        impl Allocatable for $name {
            fn cmd(&mut self) -> &mut Self {
                self.cmd = $cmd;
                self
            }

            fn len(&mut self, len: usize) -> &mut Self {
                self.tag = (len as u32) << 24;
                self
            }
        }

        impl $name {
            pub fn packet(&mut self) -> Packet<Self> {
                Packet(self as *mut Self)
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
