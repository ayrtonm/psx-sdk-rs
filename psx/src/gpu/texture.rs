use crate::gpu::vertex::Pixel;
use crate::gpu::AsU32;

pub struct Coord {
    x: u8,
    y: u8,
}

impl AsU32 for Coord {
    fn as_u32(&self) -> u32 {
        (self.y as u32) << 8 | (self.x as u32)
    }
}

impl From<(u8, u8)> for Coord {
    fn from((x, y): (u8, u8)) -> Coord {
        Coord { x, y }
    }
}

pub struct Page {
    x: u8,
    y: u8,
}

pub type Clut = Pixel;

impl AsU32 for Page {
    // TODO: currently hardcoded to 16bpp, make this variable
    fn as_u32(&self) -> u32 {
        ((self.y as u32) << 4 | (self.x as u32) | (2 << 7)) << 16
    }
}

impl AsU32 for Clut {
    fn as_u32(&self) -> u32 {
        (*self as u32) << 16
    }
}

impl From<(u8, u8)> for Page {
    fn from((x, y): (u8, u8)) -> Page {
        Page { x, y }
    }
}
