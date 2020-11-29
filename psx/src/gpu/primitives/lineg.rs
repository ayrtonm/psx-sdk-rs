use crate::gpu::color::Color;
use crate::gpu::vertex::Vertex;

#[repr(C)]
pub struct LineG2 {
    pub tag: u32,
    pub color0: Color,
    pub cmd: u8,
    pub v0: Vertex,
    pub color1: Color,
    pub _pad: u8,
    pub v1: Vertex,
}

#[repr(C)]
pub struct ColoredVertex {
    pub c: Color,
    pub _pad: u8,
    pub v: Vertex,
}

#[repr(C)]
pub struct LineG<const N: usize> {
    pub tag: u32,
    pub colored_vertices: [ColoredVertex; N],
    pub term: u32,
}
