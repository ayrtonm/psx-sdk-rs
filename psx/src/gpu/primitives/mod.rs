use core::mem::transmute;

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
    data: [u32; N],
    next_prim: usize,
}

impl<const N: usize> Buffer<N> {
    pub fn new() -> Self {
        let data = [0; N];
        Buffer { data, next_prim: 0 }
    }

    pub(self) fn get(&mut self, n: usize) -> &mut [u32] {
        self.next_prim += n;
        self.data.split_at_mut(self.next_prim).0
    }
}

pub struct DoubleBuffer<const N: usize> {
    data0: [u32; N],
    data1: [u32; N],
    next_prim: usize,
}

impl<const N: usize> DoubleBuffer<N> {
    pub fn new() -> Self {
        let data0 = [0; N];
        let data1 = [0; N];
        DoubleBuffer { data0, data1, next_prim: 0 }
    }
    pub(self) fn get(&mut self, n: usize) -> (&mut [u32], &mut [u32]) {
        self.next_prim += n;
        (self.data0.split_at_mut(self.next_prim).0, self.data1.split_at_mut(self.next_prim).0)
    }
}

/// A depth [ordering table](http://problemkaputt.de/psx-spx.htm#gpudepthordering)
pub struct OT<const N: usize> {
    entries: [u32; N],
}

impl<const N: usize> OT<N> {
    pub fn new() -> Self {
        OT {
            entries: [0; N]
        }
    }
    pub fn get(&self, n: usize) -> &u32 {
        &self.entries[n]
    }
    pub fn add_prim(&mut self, z: usize, tag: &mut u32) -> &mut Self {
        *tag &= !0x00FF_FFFF;
        *tag |= self.entries[z];
        unsafe {
            self.entries[z] = transmute::<_, u32>(tag) & 0x00FF_FFFF;
        }
        self
    }
}
