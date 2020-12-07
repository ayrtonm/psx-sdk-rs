use core::mem::size_of;
use core::ops::{Deref, DerefMut};
use core::slice::{from_raw_parts, from_raw_parts_mut};

pub mod linef;
pub mod lineg;
pub mod polyf;
pub mod polyft;
pub mod polyg;
pub mod polygt;
pub mod sprt;
pub mod tile;

mod buffer;
mod ot;
pub use buffer::{Buffer, DoubleBuffer};
pub use ot::{DoubleOT, OT};

// These should all be replaced with their respective impl Init
impl Primitive for tile::Tile {}
impl Primitive for polyft::PolyFT4 {}

#[repr(C)]
pub struct Packet<T> {
    tag: u32,
    packet: T,
}

impl<T> Deref for Packet<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.packet
    }
}

impl<T> DerefMut for Packet<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.packet
    }
}

pub struct DoublePacket<'a, T> {
    pub(self) packet_1: &'a mut Packet<T>,
    pub(self) packet_2: &'a mut Packet<T>,
    pub(self) swapped: *const bool,
}

impl<'a, T> Deref for DoublePacket<'a, T> {
    type Target = Packet<T>;

    fn deref(&self) -> &Self::Target {
        unsafe {
            if *self.swapped {
                &self.packet_1
            } else {
                &self.packet_2
            }
        }
    }
}

impl<'a, T> DerefMut for DoublePacket<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            if *self.swapped {
                &mut self.packet_1
            } else {
                &mut self.packet_2
            }
        }
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
