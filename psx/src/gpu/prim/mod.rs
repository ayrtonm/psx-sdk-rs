use core::cell::UnsafeCell;
use core::mem::{size_of, transmute};
use core::slice::{from_raw_parts, from_raw_parts_mut};

pub mod linef;
pub mod lineg;
pub mod polyf;
pub mod polyft;
pub mod polyg;
pub mod polygt;
pub mod sprt;
pub mod tile;

// This is a submodule only while it's in development
mod double_buffer;
pub use double_buffer::DoubleBuffer;
pub use double_buffer::DoublePacket;

// These should all be moved to their respective locations
impl Primitive for tile::Tile {}
impl Primitive for polyft::PolyFT4 {}
// Is this necessary/warranted?
impl Primitive for Packet<tile::Tile> {}

#[repr(C)]
pub struct Packet<T> {
    tag: u32,
    packet: T,
}

impl<T> Packet<T> {
    pub fn packet(&mut self) -> &mut T {
        &mut self.packet
    }
}

pub trait Primitive: Sized {
    fn as_slice(&self) -> &[u32] {
        let size = size_of::<Self>() / 4;
        unsafe { from_raw_parts(self as *const Self as *const u32, size) }
    }
    // Use this to unzip a file into a buffer-allocated prim
    fn as_mut_slice(&mut self) -> &mut [u32] {
        let size = size_of::<Self>() / 4;
        unsafe { from_raw_parts_mut(self as *mut Self as *mut u32, size) }
    }
}

pub trait Init {
    fn init(&mut self);
}

impl<T> Primitive for T where T: Init {}

/// A bump allocator for a single-buffered prim array.
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
            cell: UnsafeCell::new(InnerBuffer {
                data: [0; N],
                next: 0,
            }),
        }
    }

    pub fn alloc<T: Init>(&self) -> Option<&mut Packet<T>> {
        self.generic_alloc::<Packet<T>>().map(|p| {
            p.tag = (size_of::<Packet<T>>() as u32 / 4) << 24;
            p.packet.init();
            p
        })
    }

    fn generic_alloc<T>(&self) -> Option<&mut T> {
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

    pub fn empty(&mut self) {
        unsafe {
            (*self.cell.get()).next = 0;
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

    pub fn add_prim<T: Init>(&mut self, z: usize, prim: &mut Packet<T>) -> &mut Self {
        let tag = prim as *mut _ as *mut u32;
        unsafe {
            *tag &= !0x00FF_FFFF;
            *tag |= self.entries[z];
            self.entries[z] = transmute::<_, u32>(tag) & 0x00FF_FFFF;
        }
        self
    }
}
