mod color;
mod texture;
mod vertex;

pub type Pixel = i16;
pub type Command = u8;
pub type Coordinate = (Pixel, Pixel);

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Vertex {
    pub x: Pixel,
    pub y: Pixel,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PackedVertex<const N: usize, const X: usize, const Y: usize> {
    data: [u8; N],
}

pub type Clut = PackedVertex<2, 6, 9>;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TexCoord {
    pub x: u8,
    pub y: u8,
}

pub type TexPage = PackedVertex<2, 4, 1>;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DMAMode {
    GP0 = 2,
    GPUREAD,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VideoMode {
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Bpp {
    Bit4,
    Bit8,
    Bit15,
}
