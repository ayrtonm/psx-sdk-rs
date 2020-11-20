use crate::gpu::vertex::Pixel;

pub struct TIM<'a> {
    bpp: u32,
    bit_map: BitMap<'a>,
    clut: Option<BitMap<'a>>,
}

impl<'a> TIM<'a> {
    pub fn new(src: &'a [u32]) -> Self {
        let clut = ((src[1] & 8) != 0).then_some(BitMap::new(&src[2..]));
        let (offset, clut) = match clut {
            Some((offset, clut)) => (offset, Some(clut)),
            None => (2, None),
        };
        let (_, bit_map) = BitMap::new(&src[offset..]);
        TIM {
            bpp: src[1] & 3,
            bit_map,
            clut,
        }
    }

    pub fn bit_map(&self) -> &BitMap<'a> {
        &self.bit_map
    }
}

pub struct BitMap<'a> {
    len: u32,
    origin_x: Pixel,
    origin_y: Pixel,
    width: Pixel,
    height: Pixel,
    body: &'a [u32],
}

impl<'a> BitMap<'a> {
    pub fn new(src: &'a [u32]) -> (usize, Self) {
        let len = src[0];
        let len_by_u32 = (len as usize) / 4;
        (
            len_by_u32,
            BitMap {
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
}
