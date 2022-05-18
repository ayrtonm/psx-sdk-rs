//! GPU types
use crate::hw::gpu::GP0Command;

/// Predefined colors
pub mod colors;
/// GPU primitives implementing [`GP0Command`].
pub mod primitives;
mod vertex;

type Command = u8;

/// A color with components ranging from `0` to `0xFF`.
#[repr(C)]
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

/// A color with components ranging from `0` to `0x80`.
///
/// This is used for texture-blended polygons. Note that round-tripping between
/// [`TexColor`] and [`Color`] may not give exact results.
#[repr(C)]
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TexColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

/// Error for conversions from [`Vertex`] to [`PackedVertex`].
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VertexError {
    /// The x component is too large for the [`PackedVertex`].
    InvalidX,
    /// The y component is too large for the [`PackedVertex`].
    InvalidY,
}

// This is conceptually a vector since it encodes a direction, but that term is
// overloaded so let's call it a `Vertex`.
/// An (x, y) tuple representing a vector in VRAM.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Vertex(pub i16, pub i16);

/// A packed vertex
///
/// This represents two logical values encoded in an `N`-byte array with the
/// following layout.
///
/// bits `0` to `X`: X coordinate
///
/// bits `X+1` to `X+Y`: Y coordinate
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PackedVertex<const N: usize, const X: usize, const Y: usize> {
    data: [u8; N],
}

/// A color lookup table attribute
///
/// This is represented as a two-byte packed vertex with the following layout.
///
/// bits `0` to `5`: X coordinate
///
/// bits `6` to `14`: Y coordinate
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Clut(PackedVertex<2, 6, 9>);

/// A coordinate within a VRAM texture page.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TexCoord {
    pub x: u8,
    pub y: u8,
}

/// A VRAM texture page.
///
/// This is represented as a two-byte packed vertex with the following layout
///
/// bits `0` to `3`: texture page X base
///
/// bit `4`: texture page Y base
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TexPage(PackedVertex<2, 4, 1>);

/// The GPU DMA direction.
#[derive(Debug)]
pub enum DMAMode {
    /// Data is transferred from the CPU to GP0.
    GP0 = 2,
    /// Data is transferred from GPUREAD to the CPU.
    GPUREAD,
}

/// Video display mode.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VideoMode {
    NTSC = 0,
    PAL,
}

/// Color depth.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Depth {
    /// 15-bit high-color mode
    B15 = 0,
    /// 24-bit true-color mode
    B24,
}

/// Bits per pixel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Bpp {
    /// 4 bits per pixel.
    B4,
    /// 8 bits per pixel.
    B8,
    /// 15 bits per pixel.
    B15,
}

/// A physical address in memory.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysAddr([u8; 3]);

/// A GPU linked list packet for a `T`.
///
/// Typically `T` must implement [`GP0Command`], but it may also be a series of
/// GP0 commands so this trait bound is not enforced. `Copy` is not implemented
/// for this type since the physical address of [`Packet`]s is passed to the GPU
/// DMA channel so implicitly copying this is usually not wanted behavior.
#[repr(C, align(4))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Packet<T> {
    next: PhysAddr,
    size: u8,
    /// The `T` in the linked list packet.
    pub contents: T,
}

/// An error when creating [`Packet`]s.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PacketError {
    /// The [`Packet`] contents are larger than the GPU buffer. The [`Packet`]
    /// may still cause an overflow if the GPU buffer is not empty.
    Oversized,
}

/// Display environment parameters.
pub struct DispEnv {
    pub(crate) horizontal_range: PackedVertex<3, 12, 12>,
    pub(crate) vertical_range: PackedVertex<3, 10, 10>,
    pub(crate) offset: PackedVertex<3, 10, 9>,
}

impl DispEnv {
    //pub fn new(offset: (i16, i16), (x_size, _): (i16, i16)) -> Result<Self> {
    //    let offset = PackedVertex::try_from(offset)?;
    //    let ntsc_vrange = Vertex(0x88 - (240 / 2), 0x88 + (240 / 2));
    //    let hrange = Vertex(0x260, 0x260 + (x_size * 8));

    //    let horizontal_range = PackedVertex::try_from(hrange)?;
    //    let vertical_range = PackedVertex::try_from(ntsc_vrange)?;
    //    Ok(DispEnv {
    //        horizontal_range,
    //        vertical_range,
    //        offset,
    //    })
    //}
}

/// Draw environment parameters.
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
    //pub fn new(offset: Vertex, size: Vertex, bg_color: Option<Color>) ->
    // Result<Self> {    let bg_color = bg_color.unwrap_or(BLACK);
    //    let upper_left = PackedVertex::try_from(offset)?;
    //    let lower_right = PackedVertex::try_from(offset + size)?;
    //    Ok(DrawEnv {
    //        texpage_cmd: 0xE1,
    //        upper_left_cmd: 0xE3,
    //        lower_right_cmd: 0xE4,
    //        offset_cmd: 0xE5,
    //        bg_color_cmd: 0x02,

    //        texpage: (1 << 10) | 10,

    //        upper_left,
    //        lower_right,

    //        offset: PackedVertex::try_from(offset)?,

    //        bg_color,
    //        bg_offset: offset,
    //        bg_size: size,

    //        _pad: 0,
    //    })
    //}

    //pub fn set_color(&mut self, color: Color) {
    //    self.bg_color = color;
    //}
}

impl GP0Command for DrawEnv {}
