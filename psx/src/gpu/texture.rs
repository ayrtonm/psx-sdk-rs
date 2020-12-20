#[repr(C)]
#[derive(Clone, Copy)]
pub struct TexCoord {
    x: u8,
    y: u8,
}

impl From<(u8, u8)> for TexCoord {
    #[inline(always)]
    fn from((x, y): (u8, u8)) -> Self {
        TexCoord { x, y }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Clut(u16);

impl From<Option<Clut>> for Clut {
    #[inline(always)]
    fn from(maybe_clut: Option<Clut>) -> Clut {
        maybe_clut.unwrap_or(Clut(0))
    }
}

impl From<(u8, i16)> for Clut {
    #[inline(always)]
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
#[derive(Clone, Copy)]
pub struct TexPage(u16);

impl From<(u8, u8)> for TexPage {
    #[inline(always)]
    fn from((x, y): (u8, u8)) -> Self {
        TexPage(x as u16 | ((y as u16) << 4))
    }
}

impl From<(u8, u8, Bpp)> for TexPage {
    #[inline(always)]
    fn from((x, y, bpp): (u8, u8, Bpp)) -> Self {
        TexPage(x as u16 | ((y as u16) << 4) | ((bpp as u16) << 7))
    }
}
