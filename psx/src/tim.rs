use crate::gpu::texture::{Bpp, Clut, Page};
use crate::gpu::vertex::Pixel;
use crate::gpu::DrawPort;

pub struct TIM<'a> {
    bpp: Bpp,
    bitmap: Bitmap<'a>,
    clut: Option<Bitmap<'a>>,
}

impl<'a> TIM<'a> {
    pub fn new(src: &'a [u32]) -> Self {
        let clut = ((src[1] & 8) != 0).then_some(Bitmap::new(&src[2..]));
        let (offset, clut) = match clut {
            Some((offset, clut)) => (offset + 2, Some(clut)),
            None => (2, None),
        };
        let (_, bitmap) = Bitmap::new(&src[offset..]);
        let bpp = match src[1] & 3 {
            0 => Bpp::B4,
            1 => Bpp::B8,
            2 => Bpp::B15,
            _ => unreachable!("TIM contains an invalid bpp"),
        };
        TIM { bpp, bitmap, clut }
    }

    pub fn load(&self, draw_port: &mut DrawPort) -> (Page, Option<Clut>) {
        let bmp = self.bitmap();
        draw_port.to_vram(bmp.offset(), bmp.size(), bmp.body());
        let clut = self.clut().map(|clut| {
            draw_port.to_vram(clut.offset(), clut.size(), clut.body());
            let base_x = (clut.offset().0 / 16) as u8;
            let base_y = clut.offset().1;
            (base_x, base_y).into()
        });
        let base_x = (bmp.offset().0 / 64) as u8;
        let base_y = (bmp.offset().1 / 256) as u8;
        (Page::new(base_x, base_y, self.bpp), clut)
    }

    pub fn bitmap(&self) -> &Bitmap<'a> {
        &self.bitmap
    }

    pub fn clut(&self) -> Option<&Bitmap<'a>> {
        self.clut.as_ref()
    }

    pub fn bpp(&self) -> Bpp {
        self.bpp
    }
}

pub struct Bitmap<'a> {
    len: u32,
    offset: (Pixel, Pixel),
    size: (Pixel, Pixel),
    body: &'a [u32],
}

impl<'a> Bitmap<'a> {
    pub fn new(src: &'a [u32]) -> (usize, Self) {
        let len = src[0];
        let len_by_u32 = (len as usize) / 4;
        let x = src[1] as Pixel;
        let y = (src[1] >> 16) as Pixel;
        let width = src[2] as Pixel;
        let height = (src[2] >> 16) as Pixel;
        (
            len_by_u32,
            Bitmap {
                len,
                offset: (x, y),
                size: (width, height),
                body: &src[3..len_by_u32],
            },
        )
    }

    pub fn body(&self) -> &[u32] {
        self.body
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn offset(&self) -> (Pixel, Pixel) {
        self.offset
    }

    pub fn size(&self) -> (Pixel, Pixel) {
        self.size
    }
}
