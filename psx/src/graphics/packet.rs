use core::slice;
use super::packet_size;
use core::ops::{Deref, DerefMut};

impl<'a, T, U> From<&'a mut U> for &'a mut Packet<T>
where U: Deref<Target = Packet<T>> + DerefMut
{
    fn from(u: &'a mut U) -> &'a mut Packet<T> {
        u
    }
}

#[repr(C)]
pub struct Packet<T> {
    pub(crate) tag: u32,
    pub(crate) packet: T,
}

impl<T> Packet<T> {
    pub fn new(t: T) -> Self {
        Packet {
            tag: 0x03FF_FFFF,
            packet: t,
        }
    }
    pub fn address(&self) -> *const u32 {
        &self.tag as *const u32
    }
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
    pub(crate) packet_1: &'a mut Packet<T>,
    pub(crate) packet_2: &'a mut Packet<T>,
    pub(crate) swapped: *const bool,
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
