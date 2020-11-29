use crate::gpu::color::Color;
use crate::gpu::texture::{Clut, TexCoord};
use crate::gpu::vertex::Vertex;

#[repr(C)]
pub struct Sprt {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub offset: Vertex,
    pub t0: TexCoord,
    pub clut: Clut,
    pub size: Vertex,
}

#[repr(C)]
pub struct Sprt8 {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub offset: Vertex,
    pub t0: TexCoord,
    pub clut: Clut,
}

#[repr(C)]
pub struct Sprt16 {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub offset: Vertex,
    pub t0: TexCoord,
    pub clut: Clut,
}
