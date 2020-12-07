use super::{Buffer, Init, Packet};
use crate::gpu::Color;
use crate::gpu::Vertex;

#[repr(C)]
pub struct PolyF3 {
    pub color: Color,
    cmd: u8,
    pub vertices: [Vertex; 3],
}

#[repr(C)]
pub struct PolyF4 {
    pub color: Color,
    cmd: u8,
    pub vertices: [Vertex; 4],
}

macro_rules! impl_PolyF {
    ($N:expr, $name:ident, $cmd:expr) => {
        impl Init for $name {
            fn init(&mut self) {
                self.cmd();
            }
        }
        #[allow(non_snake_case)]
        impl<const N: usize> Buffer<N> {
            pub fn $name(&self) -> Option<&mut Packet<$name>> {
                self.alloc::<$name>()
            }
        }
        impl $name {
            pub fn cmd(&mut self) -> &mut Self {
                self.cmd = $cmd;
                self
            }

            pub fn vertices<T>(&mut self, vertices: [T; $N]) -> &mut Self
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
