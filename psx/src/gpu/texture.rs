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

impl From<(u8, u16)> for Clut {
    fn from((x, y): (u8, u16)) -> Self {
        Clut((y << 6) | x as u16)
    }
}

#[repr(C)]
pub struct TexPage(u16);

impl From<(u8, u8)> for TexPage {
    fn from((x, y): (u8, u8)) -> Self {
        TexPage(x as u16 | ((y as u16) << 4))
    }
}
