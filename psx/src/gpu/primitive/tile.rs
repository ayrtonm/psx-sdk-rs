use crate::gpu::color::Color;
use crate::gpu::vertex::Vertex;

#[repr(C)]
pub struct Tile {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub offset: Vertex,
    pub size: Vertex,
}

#[repr(C)]
pub struct Tile1 {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub offset: Vertex,
}

#[repr(C)]
pub struct Tile8 {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub offset: Vertex,
}

#[repr(C)]
pub struct Tile16 {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub offset: Vertex,
}
