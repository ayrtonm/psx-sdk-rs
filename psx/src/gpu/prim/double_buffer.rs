use core::ops::{Deref, DerefMut};
use core::cell::UnsafeCell;

use super::{Buffer, Init, Packet};
use super::OT;

pub struct DoublePacket<'a, T> {
    packet_1: &'a mut Packet<T>,
    packet_2: &'a mut Packet<T>,
    swapped: *const bool,
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

// TODO: remove one instance of InnerBuffer::next. This is low priority, get a
// working double buffer first.
pub struct DoubleBuffer<const N: usize> {
    buffer_1: Buffer<N>,
    buffer_2: Buffer<N>,
    swapped: UnsafeCell<bool>,
}

impl<const N: usize> DoubleBuffer<N> {
    pub fn new() -> Self {
        DoubleBuffer {
            buffer_1: Buffer::<N>::new(),
            buffer_2: Buffer::<N>::new(),
            swapped: UnsafeCell::new(false),
        }
    }

    pub fn alloc<T: Init>(&self) -> Option<DoublePacket<T>> {
        self.buffer_1
            .alloc::<T>()
            .map(move |packet_1| {
                self.buffer_2.alloc::<T>().map(move |packet_2| unsafe {
                    DoublePacket {
                        packet_1,
                        packet_2,
                        swapped: &*self.swapped.get(),
                    }
                })
            })
            .flatten()
    }

    pub fn swap(&self) {
        unsafe {
            *self.swapped.get() = !*self.swapped.get();
        }
    }
}

pub struct DoubleOT<const N: usize> {
    ot_1: OT<N>,
    ot_2: OT<N>,
    swapped: UnsafeCell<bool>,
}

impl<const N: usize> DoubleOT<N> {
    pub fn new() -> Self {
        DoubleOT {
            ot_1: OT::new(),
            ot_2: OT::new(),
            swapped: UnsafeCell::new(false),
        }
    }
    pub fn swap(&self) -> &Self {
        unsafe {
            *self.swapped.get() = !*self.swapped.get();
        }
        self
    }
}

impl<const N: usize> Deref for DoubleOT<N> {
    type Target = OT<N>;
    fn deref(&self) -> &Self::Target {
        unsafe {
            if *self.swapped.get() {
                &self.ot_1
            } else {
                &self.ot_2
            }
        }
    }
}

impl<const N: usize> DerefMut for DoubleOT<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            if *self.swapped.get() {
                &mut self.ot_1
            } else {
                &mut self.ot_2
            }
        }
    }
}

use super::polyf::PolyF4;
use super::sprt::Sprt;
#[allow(non_snake_case)]
impl<const N: usize> DoubleBuffer<N> {
    pub fn Sprt(&self) -> Option<DoublePacket<Sprt>> {
        self.alloc::<Sprt>()
    }

    pub fn PolyF4(&self) -> Option<DoublePacket<PolyF4>> {
        self.alloc::<PolyF4>()
    }
}
