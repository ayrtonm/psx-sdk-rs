use crate::gpu::color::Color;
use crate::gpu::texture::{Clut, TexCoord, TexPage};
use crate::gpu::vertex::Vertex;

#[repr(C)]
pub struct PolyFT3 {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub v0: Vertex,
    pub t0: TexCoord,
    pub clut: Clut,
    pub v1: Vertex,
    pub t1: TexCoord,
    pub tpage: TexPage,
    pub v2: Vertex,
    pub t2: TexCoord,
    pub _pad: u16,
}

#[repr(C)]
pub struct PolyFT4 {
    //pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub v0: Vertex,
    pub t0: TexCoord,
    pub clut: Clut,
    pub v1: Vertex,
    pub t1: TexCoord,
    pub tpage: TexPage,
    pub v2: Vertex,
    pub t2: TexCoord,
    pub _pad0: u16,
    pub v3: Vertex,
    pub t3: TexCoord,
    pub _pad1: u16,
}
