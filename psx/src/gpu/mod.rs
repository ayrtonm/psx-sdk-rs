//! GPU types

use crate::gpu::colors::BLACK;
use crate::hw::gpu::GP0Command;
use core::convert::TryFrom;
use strum_macros::IntoStaticStr;

pub mod colors;
#[doc(hidden)]
pub mod packet;
pub mod primitives;
mod vertex;

type Command = u8;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoStaticStr)]
pub enum VertexError {
    InvalidX,
    InvalidY,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Vertex(pub i16, pub i16);

/// A packed vertex
///
/// This represents two logical values in an `N`-byte array. One value is
/// represented by the lowest `X` bits and the other is the next `Y` bits.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct PackedVertex<const N: usize, const X: usize, const Y: usize> {
    data: [u8; N],
}

/// A color lookup table attribute
///
/// This is represented as a two-byte packed vertex with the following layout
/// bits 0-5 X coordinate
/// bits 6-14 Y coordinate
type Clut = PackedVertex<2, 6, 9>;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TexCoord {
    pub x: u8,
    pub y: u8,
}

/// A texpage
///
/// This is represented as a two-byte packed vertex with the following layout
/// bits 0-3 texture page X base
/// bit 4 texture page Y base
type TexPage = PackedVertex<2, 4, 1>;

#[derive(Debug)]
pub enum DMAMode {
    GP0 = 2,
    GPUREAD,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VertexdeoMode {
    NTSC = 0,
    PAL,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Depth {
    /// 15-bit high-color mode
    High = 0,
    /// 24-bit true-color mode
    True,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Bpp {
    Bit4,
    Bit8,
    Bit15,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysAddr([u8; 3]);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Packet<T> {
    next: PhysAddr,
    size: u8,
    pub contents: T,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoStaticStr)]
pub enum PacketError {
    Oversized,
}

type Result<T> = core::result::Result<T, VertexError>;
pub struct DispEnv {
    pub(crate) horizontal_range: PackedVertex<3, 12, 12>,
    pub(crate) vertical_range: PackedVertex<3, 10, 10>,
    pub(crate) offset: PackedVertex<3, 10, 9>,
}

impl DispEnv {
    pub fn new(offset: Vertex, Vertex(x_size, _): Vertex) -> Result<Self> {
        let offset = PackedVertex::try_from(offset)?;
        let ntsc_vrange = Vertex(0x88 - (240 / 2), 0x88 + (240 / 2));
        let hrange = Vertex(0x260, 0x260 + (x_size * 8));

        let horizontal_range = PackedVertex::try_from(hrange)?;
        let vertical_range = PackedVertex::try_from(ntsc_vrange)?;
        Ok(DispEnv {
            horizontal_range,
            vertical_range,
            offset,
        })
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct DrawEnv {
    texpage: u16,
    _pad: u8,
    texpage_cmd: Command,

    upper_left: PackedVertex<3, 10, 9>,
    upper_left_cmd: Command,

    lower_right: PackedVertex<3, 10, 9>,
    lower_right_cmd: Command,

    offset: PackedVertex<3, 11, 11>,
    offset_cmd: Command,

    bg_color: Color,
    bg_color_cmd: Command,

    bg_offset: Vertex,
    bg_size: Vertex,
}

impl DrawEnv {
    pub fn new(offset: Vertex, size: Vertex, bg_color: Option<Color>) -> Result<Self> {
        let bg_color = bg_color.unwrap_or(BLACK);
        let upper_left = PackedVertex::try_from(offset)?;
        let lower_right = PackedVertex::try_from(offset + size)?;
        Ok(DrawEnv {
            texpage_cmd: 0xE1,
            upper_left_cmd: 0xE3,
            lower_right_cmd: 0xE4,
            offset_cmd: 0xE5,
            bg_color_cmd: 0x02,

            texpage: (1 << 10) | 10,

            upper_left,
            lower_right,

            offset: PackedVertex::try_from(offset)?,

            bg_color,
            bg_offset: offset,
            bg_size: size,

            _pad: 0,
        })
    }
}

impl GP0Command for DrawEnv {}
