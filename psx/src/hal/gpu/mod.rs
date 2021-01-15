mod gp0;
mod gp1;
mod stat;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl From<Color> for u32 {
    fn from(color: Color) -> u32 {
        color.red as u32 | (color.green as u32) << 8 | (color.blue as u32) << 16
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Vertex {
    pub x: i16,
    pub y: i16,
}

impl From<Vertex> for u32 {
    fn from(vertex: Vertex) -> u32 {
        vertex.x as u32 | (vertex.y as u32) << 16
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PackedVertex<const N: usize, const X: usize, const Y: usize> {
    data: [u8; N],
}

impl<const X: usize, const Y: usize> From<PackedVertex<3, X, Y>> for u32 {
    fn from(vertex: PackedVertex<3, X, Y>) -> u32 {
        vertex.data[0] as u32 | (vertex.data[1] as u32) << 8 | (vertex.data[2] as u32) << 16
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DMA {
    GP0 = 2,
    GPUREAD,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HorizontalRes {
    H256 = 0,
    H320,
    H512,
    H640,
    H368 = 1 << 6,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VerticalRes {
    V240 = 0,
    V480,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VideoMode {
    NTSC = 0,
    PAL,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Depth {
    High = 0,
    True,
}

const fn command(cmd: u8, other_bits: Option<u32>) -> u32 {
    let other_bits = match other_bits {
        Some(bits) => bits,
        None => 0,
    };
    (cmd as u32) << 24 | other_bits
}
