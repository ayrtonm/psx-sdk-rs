use crate::gpu::vertex::Pixel;
use crate::gpu::AsU32;

#[derive(Clone, Copy)]
pub enum Bpp {
    B4 = 0,
    B8,
    B15,
}

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

pub struct Clut {
    x: u8,
    y: Pixel,
}

impl AsU32 for Clut {
    fn as_u32(&self) -> u32 {
        ((self.y as u32) << 6 | self.x as u32) << 16
    }
}

impl From<(u8, Pixel)> for Clut {
    fn from((x, y): (u8, Pixel)) -> Self {
        Clut { x, y }
    }
}

pub struct Page {
    x: u8,
    y: u8,
    bpp: Bpp,
}

impl AsU32 for Page {
    fn as_u32(&self) -> u32 {
        ((self.bpp as u32) << 7 | (self.y as u32) << 4 | (self.x as u32)) << 16
    }
}

// Specifying only the TexPage location sets the other options to sensible
// defaults. Use `Page::new` to fully specify the TexPage attributes.
impl From<(u8, u8)> for Page {
    fn from((x, y): (u8, u8)) -> Page {
        Page {
            x,
            y,
            bpp: Bpp::B15,
        }
    }
}

impl Page {
    pub fn new(x: u8, y: u8, bpp: Bpp) -> Self {
        Page { x, y, bpp }
    }
}
