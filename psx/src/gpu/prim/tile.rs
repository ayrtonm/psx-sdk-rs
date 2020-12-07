use crate::gpu::Color;
use crate::gpu::Vertex;

#[repr(C)]
pub struct Tile {
    pub color: Color,
    pub cmd: u8,
    pub offset: Vertex,
    pub size: Vertex,
}

#[repr(C)]
pub struct Tile1 {
    pub color: Color,
    cmd: u8,
    pub offset: Vertex,
}

#[repr(C)]
pub struct Tile8 {
    pub color: Color,
    cmd: u8,
    pub offset: Vertex,
}

#[repr(C)]
pub struct Tile16 {
    pub color: Color,
    cmd: u8,
    pub offset: Vertex,
}
