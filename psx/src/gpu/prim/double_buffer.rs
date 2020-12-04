use super::{Buffer, Init, InnerBuffer, Packet};
use core::cell::UnsafeCell;

pub struct DoublePacket<'a, T> {
    packet_1: &'a mut Packet<T>,
    packet_2: &'a mut Packet<T>,
    swapped: &'a bool,
}

impl<'a, T> DoublePacket<'a, T> {
    pub fn packet(&mut self) -> &mut Packet<T> {
        if *self.swapped {
            &mut self.packet_1
        } else {
            &mut self.packet_2
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

use super::sprt::Sprt;
impl<const N: usize> DoubleBuffer<N> {
    pub fn Sprt(&self) -> Option<DoublePacket<Sprt>> {
        self.alloc::<Sprt>()
    }
}
