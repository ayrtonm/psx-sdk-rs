//! GPU types
use crate::hw::gpu::GP0Command;

/// Predefined colors
pub mod colors;
mod packet;
/// GPU primitives implementing [`GP0Command`].
pub mod primitives;
mod vertex;

pub use packet::{link_list, ordering_table};

type Command = u8;

/// The number of bytes in the GPU buffer.
pub const GPU_BUFFER_SIZE: usize = 64;

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
pub type Clut = PackedVertex<2, 6, 9>;

/// A coordinate within a VRAM texture page.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TexCoord {
    pub x: u8,
    pub y: u8,
}

/// A VRAM texture page attribute.
///
/// This is represented as a two-byte packed vertex with the following layout
///
/// bits `0` to `3`: texture page X base
///
/// bit `4`: texture page Y base
pub type TexPage = PackedVertex<2, 4, 1>;

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
    Bits15 = 0,
    /// 24-bit true-color mode
    Bits24,
}

/// Bits per pixel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Bpp {
    /// 4 bits per pixel.
    Bits4,
    /// 8 bits per pixel.
    Bits8,
    /// 15 bits per pixel.
    Bits15,
}

/// A physical address in memory.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysAddr([u8; 3]);

/// A GPU linked list packet containing a `T` which may be sent over the
/// [`dma::GPU` channel][`crate::dma::GPU`].
///
/// This is essentially a `T` with a pointer to the next packet in the linked
/// list, if any. Newly created `Packet`s always point to the end of the list.
/// To link an array of `Packet`s together use [`link_list`] or the [`dma::OTC`
/// channel][`crate::dma::OTC`]. Note that linked `Packet`s don't have to be
/// contiguous in memory and that [`Packet::insert_packet`] may be used for more
/// fine-grained control over packet linking.
///
/// Typically `T` must implement [`GP0Command`] to send a linked list over DMA.
/// However, `Packet`s may also contain a series of GP0 commands so this trait
/// bound is not enforced when creating `Packet`s. Instead
/// [`dma::GPU`][`crate::dma::GPU`] enforces that its input implement
/// [`LinkedList`][`crate::dma::LinkedList`] which is automatically implemented
/// for `Packet<T> where T: GP0Command`. To sed a linked list containg `Packet`s
/// that don't implement `GP0Command`, `LinkedList` can be implemented manually.
///
/// `Copy` is intentionally not implemented for this type since the physical
/// address of [`Packet`]s is passed to the GPU DMA channel so implicitly
/// copying this is usually not wanted behavior. [`Packet`]s may be explicitly
/// copied by calling `.clone()`.
///
/// Omitting `Copy` makes creating it more cumbersome to create `Packet` arrays,
/// but the `inline_const` feature can simplify this as follows.
///
/// ```rust
/// #![feature(inline_const)]
/// let array = [const { Packet::new(T::new()) }; N]
/// ```
#[repr(C, align(4))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Packet<T> {
    // TODO: The first word should be a union to avoid UB
    next: PhysAddr,
    size: u8,
    /// The `T` in the linked list packet.
    pub contents: T,
}

/// Display buffer parameters.
pub struct DispEnv {
    pub(crate) horizontal_range: PackedVertex<3, 12, 12>,
    pub(crate) vertical_range: PackedVertex<3, 10, 10>,
    pub(crate) offset: PackedVertex<3, 10, 9>,
}

impl DispEnv {
    /// Creates a new display buffer at `offset` in VRAM with the specified
    /// `size`.
    pub fn new(
        offset: (i16, i16), size: (i16, i16), video_mode: VideoMode,
    ) -> Result<Self, VertexError> {
        let offset = Vertex::new(offset);
        let size = Vertex::new(size);
        let offset = PackedVertex::try_from(offset)?;
        let (center, range) = if video_mode == VideoMode::NTSC {
            (0x88, 0x240)
        } else {
            (0xA3, 0x256)
        };
        let ntsc_vrange = Vertex(center - (range / 2), center + (range / 2));
        let hrange = Vertex(0x260, 0x260 + (size.0 * 8));

        let horizontal_range = PackedVertex::try_from(hrange)?;
        let vertical_range = PackedVertex::try_from(ntsc_vrange)?;
        Ok(DispEnv {
            horizontal_range,
            vertical_range,
            offset,
        })
    }
}

/// Draw buffer parameters.
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

    /// The buffer's background color.
    pub bg_color: Color,
    bg_color_cmd: Command,
    bg_offset: Vertex,
    bg_size: Vertex,
}

impl DrawEnv {
    /// Creates a new draw buffer at `offset` in VRAM with the specified `size`.
    /// Background color defaults to black if `bg_color` is `None`.
    pub fn new(
        offset: (i16, i16), size: (i16, i16), bg_color: Option<Color>,
    ) -> Result<Self, VertexError> {
        let offset = Vertex::new(offset);
        let size = Vertex::new(size);
        let bg_color = bg_color.unwrap_or(colors::BLACK);
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
