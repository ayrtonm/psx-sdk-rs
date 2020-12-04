use self::bitmap::Bitmap;
use crate::gpu::{Bpp, Clut, TexPage};

mod bitmap;

pub struct TIM<'a> {
    bpp: Bpp,
    bitmap: Bitmap<'a>,
    clut_bitmap: Option<Bitmap<'a>>,
}

impl<'a> TIM<'a> {
    pub fn new(src: &'a mut [u32]) -> Self {
        let bpp = match src[1] & 3 {
            0 => Bpp::B4,
            1 => Bpp::B8,
            2 => Bpp::B15,
            _ => unreachable!("TIM contains an invalid bpp"),
        };
        let (clut_bitmap, other) = if (src[1] & 8) != 0 {
            let (bitmap, other) = Bitmap::new(&mut src[2..]);
            (Some(bitmap), other)
        } else {
            (None, &mut src[2..])
        };
        let (bitmap, _) = Bitmap::new(other);
        TIM {
            bpp,
            bitmap,
            clut_bitmap,
        }
    }

    pub fn texpage(&self) -> TexPage {
        let bmp = self.bitmap().offset();
        ((bmp.x() / 64) as u8, (bmp.y() / 256) as u8, self.bpp()).into()
    }

    pub fn clut(&self) -> Option<Clut> {
        self.clut_bitmap().map(|clut| {
            let clut = clut.offset();
            (clut.x() as u8 / 16, clut.y()).into()
        })
    }

    pub fn bitmap(&self) -> &Bitmap<'a> {
        &self.bitmap
    }

    pub fn clut_bitmap(&self) -> Option<&Bitmap<'a>> {
        self.clut_bitmap.as_ref()
    }

    fn bpp(&self) -> Bpp {
        self.bpp
    }
}
