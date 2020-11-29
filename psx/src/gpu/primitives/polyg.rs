use crate::gpu::color::Color;
use crate::gpu::vertex::Vertex;

#[repr(C)]
pub struct PolyG3 {
    pub tag: u32,
    pub color0: Color,
    pub cmd: u8,
    pub v0: Vertex,
    pub color1: Color,
    pub _pad0: u8,
    pub v1: Vertex,
    pub color2: Color,
    pub _pad1: u8,
    pub v2: Vertex,
}

#[repr(C)]
pub struct PolyG4 {
    pub tag: u32,
    pub color0: Color,
    pub cmd: u8,
    pub v0: Vertex,
    pub color1: Color,
    pub _pad0: u8,
    pub v1: Vertex,
    pub color2: Color,
    pub _pad1: u8,
    pub v2: Vertex,
    pub color3: Color,
    pub _pad2: u8,
    pub v3: Vertex,
}
