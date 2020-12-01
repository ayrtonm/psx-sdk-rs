use core::cell::UnsafeCell;
use core::mem::{size_of, transmute};

pub mod linef;
pub mod lineg;
pub mod polyf;
pub mod polyft;
pub mod polyg;
pub mod polygt;
pub mod sprt;
pub mod tile;

impl Primitive for tile::Tile {}

pub trait Primitive: Sized {
    fn as_slice(&self) -> &[u32] {
        let size = size_of::<Self>() / 4;
        unsafe { &core::slice::from_raw_parts(self as *const Self as *const u32, size)[1..] }
    }
}

/// A bump allocator for a single-buffered primitive array.
pub struct Buffer<const N: usize> {
    cell: UnsafeCell<InnerBuffer<N>>,
}

struct InnerBuffer<const N: usize> {
    data: [u32; N],
    next: usize,
}

impl<const N: usize> Buffer<N> {
    pub fn new() -> Self {
        Buffer {
            cell: UnsafeCell::new(InnerBuffer::new()),
        }
    }

    pub fn alloc<T>(&self) -> Option<&mut T> {
        unsafe {
            let size = size_of::<T>() / 4;
            let start = (*self.cell.get()).next;
            let end = start + size;
            if end < N {
                (*self.cell.get()).next += size;
                let slice = &mut (*self.cell.get()).data[start..end];
                let ptr = slice.as_mut_ptr().cast::<T>();
                ptr.as_mut()
            } else {
                None
            }
        }
    }
}

impl<const N: usize> InnerBuffer<N> {
    pub fn new() -> Self {
        InnerBuffer {
            data: [0; N],
            next: 0,
        }
    }
}

/// A depth [ordering table](http://problemkaputt.de/psx-spx.htm#gpudepthordering)
pub struct OT<const N: usize> {
    entries: [u32; N],
}

impl<const N: usize> OT<N> {
    pub fn new() -> Self {
        OT { entries: [0; N] }
    }

    pub fn len(&self) -> usize {
        N - 1
    }

    pub fn start(&self) -> &u32 {
        &self.entries[N - 1]
    }

    pub fn entry(&self, n: usize) -> &u32 {
        &self.entries[n]
    }

    pub fn add_prim<T: Primitive>(&mut self, z: usize, prim: &mut T) -> &mut Self {
        let tag = prim as *mut _ as *mut u32;
        unsafe {
            *tag &= !0x00FF_FFFF;
            *tag |= self.entries[z];
            self.entries[z] = transmute::<_, u32>(tag) & 0x00FF_FFFF;
        }
        self
    }
}
