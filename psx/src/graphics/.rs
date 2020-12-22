use crate::gpu::{Clut, Color, TexCoord, TexPage, Vertex};

#[repr(C)]
pub struct PolyF3 {
    pub color: Color,
    pub(crate) cmd: u8,
    pub v0: Vertex,
    pub v1: Vertex,
    pub v2: Vertex,
}

#[repr(C)]
pub struct PolyF4 {
    pub color: Color,
    pub(crate) cmd: u8,
    pub v0: Vertex,
    pub v1: Vertex,
    pub v2: Vertex,
    pub v3: Vertex,
}

#[repr(C)]
pub struct PolyFT3 {
    pub color: Color,
    pub(crate) cmd: u8,
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
    pub color: Color,
    pub(crate) cmd: u8,
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

#[repr(C)]
pub struct PolyG3 {
    pub color0: Color,
    pub(crate) cmd: u8,
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
    pub color0: Color,
    pub(crate) cmd: u8,
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

#[repr(C)]
pub struct PolyGT3 {
    pub color0: Color,
    pub(crate) cmd: u8,
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
    pub color0: Color,
    pub(crate) cmd: u8,
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

#[repr(C)]
pub struct LineF2 {
    pub color: Color,
    pub(crate) cmd: u8,
    pub v0: Vertex,
    pub v1: Vertex,
}

#[repr(C)]
pub struct LineF<const N: usize> {
    pub color: Color,
    pub(crate) cmd: u8,
    pub vertices: [Vertex; N],
    term: u32,
}

#[repr(C)]
pub struct LineG2 {
    pub color0: Color,
    pub(crate) cmd: u8,
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

#[repr(C)]
pub struct Tile {
    pub color: Color,
    pub(crate) cmd: u8,
    pub offset: Vertex,
    pub size: Vertex,
}

#[repr(C)]
pub struct Tile1 {
    pub color: Color,
    pub(crate) cmd: u8,
    pub offset: Vertex,
}

#[repr(C)]
pub struct Tile8 {
    pub color: Color,
    pub(crate) cmd: u8,
    pub offset: Vertex,
}

#[repr(C)]
pub struct Tile16 {
    pub color: Color,
    pub(crate) cmd: u8,
    pub offset: Vertex,
}

#[repr(C)]
pub struct Sprt {
    pub color: Color,
    pub(crate) cmd: u8,
    pub offset: Vertex,
    pub t0: TexCoord,
    pub clut: Clut,
    pub size: Vertex,
}

#[repr(C)]
pub struct Sprt8 {
    pub color: Color,
    pub(crate) cmd: u8,
    pub offset: Vertex,
    pub t0: TexCoord,
    pub clut: Clut,
}

#[repr(C)]
pub struct Sprt16 {
    pub color: Color,
    pub(crate) cmd: u8,
    pub offset: Vertex,
    pub t0: TexCoord,
    pub clut: Clut,
}
