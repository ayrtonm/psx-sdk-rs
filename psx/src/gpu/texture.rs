#[repr(C)]
pub struct TexCoord {
    x: u8,
    y: u8,
}

impl From<(u8, u8)> for TexCoord {
    fn from((x, y): (u8, u8)) -> Self {
        TexCoord { x, y }
    }
}

#[repr(C)]
pub struct Clut(u16);

impl From<(u8, i16)> for Clut {
    fn from((x, y): (u8, i16)) -> Self {
        Clut((y << 6) as u16 | x as u16)
    }
}

#[derive(Clone, Copy)]
pub enum Bpp {
    B4 = 0,
    B8,
    B15,
}

#[repr(C)]
pub struct TexPage(u16);

impl From<(u8, u8)> for TexPage {
    fn from((x, y): (u8, u8)) -> Self {
        TexPage(x as u16 | ((y as u16) << 4))
    }
}

impl From<(u8, u8, Bpp)> for TexPage {
    fn from((x, y, bpp): (u8, u8, Bpp)) -> Self {
        TexPage(x as u16 | ((y as u16) << 4) | ((bpp as u16) << 7))
    }
}
