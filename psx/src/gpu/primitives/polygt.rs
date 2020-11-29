use crate::gpu::color::Color;
use crate::gpu::texture::{Clut, TexCoord, TexPage};
use crate::gpu::vertex::Vertex;

#[repr(C)]
pub struct PolyGT3 {
    pub tag: u32,
    pub color0: Color,
    pub cmd: u8,
    pub v0: Vertex,
    pub t0: TexCoord,
    pub clut: Clut,
    pub color1: Color,
    pub _pad0: u8,
    pub v1: Vertex,
    pub t1: TexCoord,
    pub tpage: TexPage,
    pub color2: Color,
    pub _pad1: u8,
    pub v2: Vertex,
    pub t2: TexCoord,
    pub _pad2: u16,
}

#[repr(C)]
pub struct PolyGT4 {
    pub tag: u32,
    pub color0: Color,
    pub cmd: u8,
    pub v0: Vertex,
    pub t0: TexCoord,
    pub clut: Clut,
    pub color1: Color,
    pub _pad0: u8,
    pub v1: Vertex,
    pub t1: TexCoord,
    pub tpage: TexPage,
    pub color2: Color,
    pub _pad1: u8,
    pub v2: Vertex,
    pub t2: TexCoord,
    pub _pad2: u16,
    pub color3: Color,
    pub _pad3: u8,
    pub v3: Vertex,
    pub t3: TexCoord,
    pub _pad4: u16,
}
