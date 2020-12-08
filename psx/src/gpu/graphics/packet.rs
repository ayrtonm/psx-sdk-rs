use core::ops::{Deref, DerefMut};

#[repr(C)]
pub struct SinglePacket<T> {
    pub(crate) tag: u32,
    pub(crate) packet: T,
}

impl<T> Deref for SinglePacket<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.packet
    }
}

impl<T> DerefMut for SinglePacket<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.packet
    }
}

pub struct DoublePacket<'a, T> {
    pub(crate) packet_1: &'a mut SinglePacket<T>,
    pub(crate) packet_2: &'a mut SinglePacket<T>,
    pub(crate) swapped: *const bool,
}

impl<'a, T> Deref for DoublePacket<'a, T> {
    type Target = SinglePacket<T>;

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
