pub mod linef;
pub mod lineg;
pub mod polyf;
pub mod polyft;
pub mod polyg;
pub mod polygt;
pub mod sprt;
pub mod tile;

/// A bump allocator for a single-buffered primitive array.
pub struct Buffer<const N: usize> {
    pub data: [u32; N],
    pub nextpri: usize,
}

impl<const N: usize> Buffer<N> {
    pub fn new() -> Self {
        let data = [0; N];
        Buffer { data, nextpri: 0 }
    }

    pub(self) fn get(&mut self, n: usize) -> &mut [u32] {
        self.nextpri += n;
        self.data.split_at_mut(self.nextpri).0
    }
}
