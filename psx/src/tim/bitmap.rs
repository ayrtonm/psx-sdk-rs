use crate::gpu::{Pixel, Vertex};

pub struct Bitmap<'a> {
    data: &'a [u32],
}

impl<'a> Bitmap<'a> {
    pub fn new(src: &'a mut [u32]) -> (Self, &'a mut [u32]) {
        let words = src[0] as usize / 4;
        src[0] = 0xA0 << 24;
        let (data, other) = src.split_at_mut(words);
        (Bitmap { data }, other)
    }

    pub fn data(&self) -> &[u32] {
        self.data
    }

    pub(super) fn offset(&self) -> Vertex {
        (self.data[1] as Pixel, (self.data[1] >> 16) as Pixel).into()
    }

    //fn body(&self) -> &[u32] {
    //    &self.data[2..]
    //}

    //fn len(&self) -> u32 {
    //    self.data[0]
    //}

    //fn size(&self) -> Vertex {
    //    (self.data[2] as Pixel, (self.data[2] >> 16) as Pixel).into()
    //}
}
