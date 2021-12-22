use crate::gpu::Bpp;
use strum_macros::IntoStaticStr;

pub struct TIM<'a> {
    bpp: Bpp,
    bmp: Bitmap<'a>,
    clut_bmp: Option<Bitmap<'a>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoStaticStr)]
pub enum Error {
    MissingMagic,
    BadMagic,
    MissingBpp,
    InvalidBpp,
    MissingData,
}

impl<'a> TIM<'a> {
    pub fn new(src: &'a mut [u32]) -> Result<Self, Error> {
        const MAGIC: u32 = 0x0000_0010;
        if src.get(0).ok_or(Error::MissingMagic)? != MAGIC {
            return Err(Error::BadMagic);
        }
        let flags = src.get(1).ok_or(Error::MissingBpp)?;
        let bpp = match flags & 0b11 {
            0 => Bpp::Bit4,
            1 => Bpp::Bit8,
            2 => Bpp::Bit15,
            _ => return Err(Error::InvalidBpp),
        };
        if src.len() < 3 {
            return Err(Error::MissingData);
        }
        let (clut_bmp, other) = if (flags & 8) != 0 {
            let (bmp, other) = Bitmap::new(&mut src[2..]);
            (Some(bmp), other)
        } else {
            (None, &mut src[2..])
        };
        let (bmp, _) = Bitmap::new(other);
        Ok(TIM { bpp, bmp, clut_bmp })
    }
}

struct Bitmap<'a>(&'a [u32]);

impl<'a> Bitmap<'a> {
    pub fn new(src: &'a mut [u32]) -> (Self, &'a mut [u32]) {
        let words = src[0] / 4;
        src[0] = 0xA0 << 24;
        let (data, other) = src.split_at_mut(words as usize);
        (Bitmap(data), other)
    }
}
