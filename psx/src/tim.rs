//! Parser for texture data in TIM format

#![allow(dead_code)]
use crate::gpu::{Bpp, Clut, Pixel, TexPage, Vertex};

pub struct TIM<'a> {
    bpp: Bpp,
    bmp: Bitmap<'a>,
    clut_bmp: Option<Bitmap<'a>>,
}

impl<'a> TIM<'a> {
    pub fn new(src: &'a mut [u32]) -> Self {
        let bpp = match src[1] & 0b11 {
            0 => Bpp::Bit4,
            1 => Bpp::Bit8,
            2 => Bpp::Bit15,
            3 => illegal!("TIM contained an invalid bpp\0"),
            _ => {
                // This is OK since `& 0b11` in the matched expr restricts its value to [0, 3]
                unsafe { core::hint::unreachable_unchecked() }
            },
        };
        let (clut_bmp, other) = if (src[1] & 8) != 0 {
            let (bmp, other) = Bitmap::new(&mut src[2..]);
            (Some(bmp), other)
        } else {
            (None, &mut src[2..])
        };
        let (bmp, _) = Bitmap::new(other);
        TIM { bpp, bmp, clut_bmp }
    }

    pub fn tex_page(&self) -> TexPage {
        let bmp = self.bmp.offset();
        ((bmp.x / 64) % 0xFF, (bmp.y / 256) % 0xFF).into()
    }

    pub fn clut(&self) -> Option<Clut> {
        self.clut_bmp.as_ref().map(|clut| {
            let clut = clut.offset();
            ((clut.x & 0xFF) / 16, clut.y).into()
        })
    }

    pub fn bmp(&self) -> &[u32] {
        self.bmp.data()
    }

    pub fn clut_bmp(&self) -> Option<&[u32]> {
        self.clut_bmp.as_ref().map(|clut_bmp| clut_bmp.data())
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

    pub fn data(&self) -> &[u32] {
        self.0
    }

    fn offset(&self) -> Vertex {
        (self.0[1] as Pixel, (self.0[1] >> 16) as Pixel).into()
    }
}
