use crate::gpu::vertex::{Pixel, Vertex};

pub struct Bitmap<'a> {
    data: &'a [u32],
}

impl<'a> Bitmap<'a> {
    pub fn new(src: &'a [u32]) -> (usize, Self) {
        let len_by_u32 = (src[0] as usize) / 4;
        (
            len_by_u32,
            Bitmap {
                data: &src[1..len_by_u32],
            },
        )
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
