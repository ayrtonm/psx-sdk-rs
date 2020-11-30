use core::mem::{size_of, transmute};

pub mod linef;
pub mod lineg;
pub mod polyf;
pub mod polyft;
pub mod polyg;
pub mod polygt;
pub mod sprt;
pub mod tile;

// TODO: move this to self::tile
impl Primitive for tile::Tile {}

pub trait Primitive: Sized {
    fn as_slice(&self) -> &[u32] {
        let size = size_of::<Self>() / 4;
        unsafe {
            &core::slice::from_raw_parts(self as *const Self as *const u32, size)[1..]
        }
    }
}

pub struct Packet<T>(*mut T);
impl<T> Packet<T> {
    pub fn as_mut(&self) -> &mut T {
        unsafe { self.0.as_mut().unwrap() }
    }
}
pub trait Allocatable: Sized {
    fn cmd(&mut self) -> &mut Self;
    fn len(&mut self, len: usize) -> &mut Self;
}
/// A bump allocator for a single-buffered primitive array.
pub struct Buffer<const N: usize> {
    pub data: [u32; N],
    next_primitive: usize,
}

impl<const N: usize> Buffer<N> {
    pub fn new() -> Self {
        let data = [0; N];
        Buffer {
            data,
            next_primitive: 0,
        }
    }

    pub fn alloc<T: Allocatable>(&mut self) -> Packet<T> {
        let slice = self.get(size_of::<T>() / 4);
        let ptr = slice.as_mut_ptr().cast::<T>();
        let prim = Packet(ptr);
        prim.as_mut().cmd().len(size_of::<T>() / 4);
        prim
    }

    fn get(&mut self, n: usize) -> &mut [u32] {
        let start = self.next_primitive;
        let end = self.next_primitive + n;
        self.next_primitive = end;
        &mut self.data[start..end]
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

    pub fn start(&self) -> &u32 {
        &self.entries[N - 1]
    }

    pub fn entry(&self, n: usize) -> &u32 {
        &self.entries[n]
    }

    pub fn add_prim<T>(&mut self, z: usize, prim: Packet<T>) -> &mut Self {
        let tag = prim.as_mut() as *mut _ as *mut u32;
        unsafe {
            *tag &= !0x00FF_FFFF;
            *tag |= self.entries[z];
            self.entries[z] = transmute::<_, u32>(tag) & 0x00FF_FFFF;
        }
        self
    }
}
