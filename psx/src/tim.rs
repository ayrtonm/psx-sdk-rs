use crate::gpu::vertex::Pixel;

pub struct TIM<'a> {
    bpp: u32,
    bitmap: Bitmap<'a>,
    clut: Option<Bitmap<'a>>,
}

impl<'a> TIM<'a> {
    pub fn new(src: &'a [u32]) -> Self {
        let clut = ((src[1] & 8) != 0).then_some(Bitmap::new(&src[2..]));
        let (offset, clut) = match clut {
            Some((offset, clut)) => (offset, Some(clut)),
            None => (2, None),
        };
        let (_, bitmap) = Bitmap::new(&src[offset..]);
        TIM {
            bpp: src[1] & 3,
            bitmap,
            clut,
        }
    }

    pub fn bitmap(&self) -> &Bitmap<'a> {
        &self.bitmap
    }

    pub fn clut(&self) -> Option<&Bitmap<'a>> {
        self.clut.as_ref()
    }

    pub fn bpp(&self) -> u32 {
        self.bpp
    }
}

pub struct Bitmap<'a> {
    len: u32,
    origin_x: Pixel,
    origin_y: Pixel,
    width: Pixel,
    height: Pixel,
    body: &'a [u32],
}

impl<'a> Bitmap<'a> {
    pub fn new(src: &'a [u32]) -> (usize, Self) {
        let len = src[0];
        let len_by_u32 = (len as usize) / 4;
        (
            len_by_u32,
            Bitmap {
                len,
                origin_x: src[1] as Pixel,
                origin_y: (src[1] >> 16) as Pixel,
                width: src[2] as Pixel,
                height: (src[2] >> 16) as Pixel,
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

    pub fn origin_x(&self) -> Pixel {
        self.origin_x
    }

    pub fn origin_y(&self) -> Pixel {
        self.origin_y
    }

    pub fn width(&self) -> Pixel {
        self.width
    }

    pub fn height(&self) -> Pixel {
        self.height
    }
}
