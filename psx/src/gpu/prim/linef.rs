use crate::gpu::Color;
use crate::gpu::Vertex;

#[repr(C)]
pub struct LineF2 {
    pub color: Color,
    cmd: u8,
    pub v0: Vertex,
    pub v1: Vertex,
}

#[repr(C)]
pub struct LineF<const N: usize> {
    pub color: Color,
    cmd: u8,
    pub vertices: [Vertex; N],
    term: u32,
}
