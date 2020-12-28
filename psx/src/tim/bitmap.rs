use crate::gpu::{Pixel, Vertex};

pub struct Bitmap<'a>(&'a [u32]);

impl<'a> Bitmap<'a> {
    pub fn new(src: &'a mut [u32]) -> (Self, &'a mut [u32]) {
        let words = src[0] / 4;
        src[0] = 0xA0 << 24;
        let (data, other) = src.split_at_mut(words as usize);
        (Bitmap(data), other)
    }

    pub fn offset(&self) -> Vertex {
        (self.0[1] as Pixel, (self.0[1] >> 16) as Pixel).into()
    }
}
