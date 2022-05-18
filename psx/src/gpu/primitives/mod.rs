use crate::gpu::{Clut, Color, Command, TexColor, TexCoord, TexPage, Vertex};
use crate::hw::gpu::GP0Command;
use core::mem::{size_of, transmute};

#[macro_use]
mod macros;

/// Flat-shaded, non-textured triangle.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PolyF3 {
    color: Color,
    cmd: Command,
    v0: Vertex,
    v1: Vertex,
    v2: Vertex,
}

/// Flat-shaded, non-textured quad.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PolyF4 {
    color: Color,
    cmd: Command,
    v0: Vertex,
    v1: Vertex,
    v2: Vertex,
    v3: Vertex,
}

/// Flat-shaded, textured triangle.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PolyFT3 {
    color: TexColor,
    cmd: Command,
    v0: Vertex,
    t0: TexCoord,
    clut: Clut,
    v1: Vertex,
    t1: TexCoord,
    tpage: TexPage,
    v2: Vertex,
    t2: TexCoord,
    _pad: u16,
}

/// Flat-shaded, textured quad.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PolyFT4 {
    color: TexColor,
    cmd: Command,
    v0: Vertex,
    t0: TexCoord,
    clut: Clut,
    v1: Vertex,
    t1: TexCoord,
    tpage: TexPage,
    v2: Vertex,
    t2: TexCoord,
    _pad0: u16,
    v3: Vertex,
    t3: TexCoord,
    _pad1: u16,
}

/// Gouraud-shaded, non-textured triangle.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PolyG3 {
    color0: Color,
    cmd: Command,
    v0: Vertex,
    color1: Color,
    _pad0: u8,
    v1: Vertex,
    color2: Color,
    _pad1: u8,
    v2: Vertex,
}

/// Gouraud-shaded, non-textured quad.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PolyG4 {
    color0: Color,
    cmd: Command,
    v0: Vertex,
    color1: Color,
    _pad0: u8,
    v1: Vertex,
    color2: Color,
    _pad1: u8,
    v2: Vertex,
    color3: Color,
    _pad2: u8,
    v3: Vertex,
}

/// Gouraud-shaded, textured triangle.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PolyGT3 {
    color0: TexColor,
    cmd: Command,
    v0: Vertex,
    t0: TexCoord,
    clut: Clut,
    color1: TexColor,
    _pad0: u8,
    v1: Vertex,
    t1: TexCoord,
    tpage: TexPage,
    color2: TexColor,
    _pad1: u8,
    v2: Vertex,
    t2: TexCoord,
    _pad2: u16,
}

/// Gouraud-shaded, textured quad.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PolyGT4 {
    color0: TexColor,
    cmd: Command,
    v0: Vertex,
    t0: TexCoord,
    clut: Clut,
    color1: TexColor,
    _pad0: u8,
    v1: Vertex,
    t1: TexCoord,
    tpage: TexPage,
    color2: TexColor,
    _pad1: u8,
    v2: Vertex,
    t2: TexCoord,
    _pad2: u16,
    color3: TexColor,
    _pad3: u8,
    v3: Vertex,
    t3: TexCoord,
    _pad4: u16,
}

/// Flat-shaded line.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LineF2 {
    color: Color,
    cmd: Command,
    v0: Vertex,
    v1: Vertex,
}

/// Flat-shaded poly-line.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LineF<const N: usize> {
    color: Color,
    cmd: Command,
    vertices: [Vertex; N],
    term: u32,
}

/// Gouraud-shaded line.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LineG2 {
    color0: Color,
    cmd: Command,
    v0: Vertex,
    color1: Color,
    _pad: u8,
    v1: Vertex,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct ColoredVertex {
    c: Color,
    _pad: u8,
    v: Vertex,
}

/// Gouraud-shaded poly-line.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LineG<const N: usize> {
    colored_vertices: [ColoredVertex; N],
    term: u32,
}

/// Monochrome rectangle.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Tile {
    color: Color,
    cmd: Command,
    offset: Vertex,
    size: Vertex,
}

/// Monochrome 1x1 rectangle.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Tile1 {
    color: Color,
    cmd: Command,
    offset: Vertex,
}

/// Monochrome 8x8 rectangle.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Tile8 {
    color: Color,
    cmd: Command,
    offset: Vertex,
}

/// Monochrome 16x16 rectangle.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Tile16 {
    color: Color,
    cmd: Command,
    offset: Vertex,
}

/// Textured rectangle.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Sprt {
    color: TexColor,
    cmd: Command,
    offset: Vertex,
    t0: TexCoord,
    clut: Clut,
    size: Vertex,
}

/// Textured 8x8 rectangle.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Sprt8 {
    color: TexColor,
    cmd: Command,
    offset: Vertex,
    t0: TexCoord,
    clut: Clut,
}

/// Textured 16x16 rectangle.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Sprt16 {
    color: TexColor,
    cmd: Command,
    offset: Vertex,
    t0: TexCoord,
    clut: Clut,
}

impl_primitive!(PolyF3, 0x20);
impl PolyF3 {
    vertices_fn!(3);
    color_fn!();
}
impl_primitive!(PolyF4, 0x28);
impl PolyF4 {
    vertices_fn!(4);
    color_fn!();
}
impl_primitive!(PolyFT3, 0x24);
impl PolyFT3 {
    vertices_fn!(3);
    color_fn!(textured);
    //clut_fn!();
    //tex_page_fn!();
    //tex_coord_fn!(3);
}
impl_primitive!(PolyFT4, 0x2C);
impl PolyFT4 {
    vertices_fn!(4);
    color_fn!(textured);
    //clut_fn!();
    //tex_page_fn!();
    //tex_coord_fn!(4);
}
impl_primitive!(PolyG3, 0x30);
impl PolyG3 {
    vertices_fn!(3);
    gouraud_fn!(3);
}
impl_primitive!(PolyG4, 0x38);
impl PolyG4 {
    vertices_fn!(4);
    gouraud_fn!(4);
}
impl_primitive!(PolyGT3, 0x34);
impl PolyGT3 {
    vertices_fn!(3);
    gouraud_fn!(3, textured);
    //clut_fn!();
    //tex_page_fn!();
    //tex_coord_fn!(3);
}
impl_primitive!(PolyGT4, 0x3C);
impl PolyGT4 {
    vertices_fn!(4);
    gouraud_fn!(4, textured);
    //clut_fn!();
    //tex_page_fn!();
    //tex_coord_fn!(4);
}
impl_primitive!(LineF2, 0x40);
//impl_primitive!(LineF<N>, 0x48);
impl_primitive!(LineG2, 0x50);
//impl_primitive!(LineG<N>, 0x58);
impl_primitive!(Tile, 0x60);
impl_primitive!(Tile1, 0x68);
impl_primitive!(Tile8, 0x70);
impl Tile8 {
    color_fn!();
    offset_fn!();
}
impl_primitive!(Tile16, 0x78);
impl_primitive!(Sprt, 0x64);
impl Sprt {
    color_fn!(textured);
    offset_fn!();
    size_fn!();
    clut_fn!();
    tex_coord_fn!(1);
}
impl_primitive!(Sprt8, 0x74);
impl Sprt8 {
    color_fn!(textured);
    offset_fn!();
    clut_fn!();
    tex_coord_fn!(1);
}
impl_primitive!(Sprt16, 0x7C);
impl Sprt16 {
    color_fn!(textured);
    offset_fn!();
    clut_fn!();
    tex_coord_fn!(1);
}
