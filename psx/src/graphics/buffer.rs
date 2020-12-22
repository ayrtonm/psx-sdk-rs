use core::cell::UnsafeCell;
use core::mem::{size_of, MaybeUninit};

use super::{packet_size, DoublePacket, Packet, Primitive};

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

    pub fn alloc<T: Primitive>(&self) -> Option<&mut Packet<T>> {
        self.generic_alloc::<Packet<T>>().map(|p| {
            p.tag = (packet_size::<T>() << 24) as u32;
            p.packet.init();
            p
        })
    }

    fn generic_alloc<T>(&self) -> Option<&mut T> {
        unsafe {
            let size = size_of::<T>() / 4;
            let start = (*self.cell.get()).next;
            let end = start + size;
            if end <= N {
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

    pub fn alloc_array<T: Primitive, const M: usize>(&self) -> Option<[&mut Packet<T>; M]> {
        let mut ar: [&mut Packet<T>; M] = unsafe { MaybeUninit::zeroed().assume_init() };
        for i in 0..M {
            self.alloc().map(|t| ar[i] = t).or_else(|| return None);
        }
        Some(ar)
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

    // TODO: should alloc return Option<T>? One on hand, the buffer size is const so
    // you should be able to plan on being able to make some max number of
    // primitives. On the other hand, this could be useful for knowing when to
    // reset the buffer without having to plan so carefully.
    pub fn alloc<T: Primitive>(&self) -> Option<DoublePacket<T>> {
        let opt = self
            .buffer_1
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
            .flatten();
        // TODO: Fix this ugly hack after implementing pretty panic
        if opt.is_none() {
            panic!("primitive buffer overflow");
        };
        opt
    }

    pub fn swap(&self) {
        unsafe {
            *self.swapped.get() = !*self.swapped.get();
        }
    }

    pub fn alloc_array<T: Primitive, const M: usize>(&self) -> Option<[DoublePacket<T>; M]> {
        let mut ar: [DoublePacket<T>; M] = unsafe { MaybeUninit::zeroed().assume_init() };
        for i in 0..M {
            self.alloc().map(|t| ar[i] = t).or_else(|| return None);
        }
        Some(ar)
    }
}
