use crate::gpu::{Pixel, Vertex};
use crate::workarounds::split_at_mut;

pub struct Bitmap<'a>(&'a [u32]);

// TODO: why is unchecked ok here?
impl<'a> Bitmap<'a> {
    pub fn new(src: &'a mut [u32]) -> (Self, &'a mut [u32]) {
        let words = unsafe { src.get_unchecked(0) } / 4;
        unsafe { *src.get_unchecked_mut(0) = 0xA0 << 24 };
        let (data, other) = unsafe { split_at_mut(src, words as usize) };
        (Bitmap(data), other)
    }

    pub fn offset(&self) -> Vertex {
        unsafe {
            (
                *self.0.get_unchecked(1) as Pixel,
                (*self.0.get_unchecked(1) >> 16) as Pixel,
            )
                .into()
        }
    }

    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    pub fn data(&self) -> &[u32] {
        self.0
    }
}
