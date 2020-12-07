use crate::gpu::Color;
use crate::gpu::Vertex;

#[repr(C)]
pub struct LineG2 {
    pub color0: Color,
    cmd: u8,
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
    pub colored_vertices: [ColoredVertex; N],
    term: u32,
}
