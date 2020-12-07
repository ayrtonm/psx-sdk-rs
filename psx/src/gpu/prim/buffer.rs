use core::cell::UnsafeCell;
use core::mem::size_of;

use super::{DoublePacket, Init, Packet};

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
