use crate::gpu::color::Color;
use crate::gpu::vertex::Vertex;

#[repr(C)]
pub struct LineF2 {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub v0: Vertex,
    pub v1: Vertex,
}

#[repr(C)]
pub struct LineF<const N: usize> {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub vertices: [Vertex; N],
    pub term: u32,
}
