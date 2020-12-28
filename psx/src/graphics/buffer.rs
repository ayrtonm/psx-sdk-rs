use core::cell::UnsafeCell;
use core::mem::{size_of, MaybeUninit};

use super::InitPrimitive;

use crate::graphics::packet::{DoublePacket, Packet};

/// An `N`-word bump allocator a single-buffered primitives.
pub struct Buffer<const N: usize>(UnsafeCell<InnerBuffer<N>>);

struct InnerBuffer<const N: usize> {
    data: [u32; N],
    next: usize,
}

impl<const N: usize> Buffer<N> {
    /// Constructs a new single-buffered primitive bump allocator.
    pub fn new() -> Self {
        Buffer(UnsafeCell::new(InnerBuffer {
            data: [0; N],
            next: 0,
        }))
    }

    /// Empties the buffer invalidating previously allocated primitives.
    /// Allocated data remains intact until new allocations overwrite it.
    pub fn empty(&mut self) -> &mut Self {
        unsafe {
            (*self.0.get()).next = 0;
        }
        self
    }

    /// Gets the number of words remaining in the buffer.
    pub fn words_remaining(&self) -> usize {
        unsafe { N - (*self.0.get()).next }
    }

    /// Allocate a packet of type `T`. Returns `None` if remaining buffer space
    /// is insufficient.
    pub fn alloc<T: InitPrimitive>(&self) -> Option<&mut Packet<T>> {
        self.generic_alloc::<Packet<T>>().map(|p| {
            let packet_words = size_of::<Packet<T>>() / 4;
            p.tag = (packet_words << 24) as u32;
            p.data.init_primitive();
            p
        })
    }

    /// Allocates an array of packets of type `T`. Returns `None` if remaining
    /// buffer space is insufficient.
    pub fn alloc_array<T: InitPrimitive, const M: usize>(&self) -> Option<[&mut Packet<T>; M]> {
        let mut ar: [&mut Packet<T>; M] = unsafe { MaybeUninit::zeroed().assume_init() };
        for i in 0..M {
            self.alloc().map(|p| ar[i] = p).or_else(|| return None);
        }
        Some(ar)
    }

    fn generic_alloc<T>(&self) -> Option<&mut T> {
        let words = size_of::<T>() / 4;
        let start = unsafe { (*self.0.get()).next };
        let end = start + words;
        if end <= N {
            unsafe {
                (*self.0.get()).next += words;
                let slice = &mut (*self.0.get()).data[start..end];
                let ptr = slice.as_mut_ptr().cast::<T>();
                ptr.as_mut()
            }
        } else {
            None
        }
    }
}

/// A `2N`-word bump allocator a double-buffered primitives.
pub struct DoubleBuffer<const N: usize> {
    buffer_0: Buffer<N>,
    buffer_1: Buffer<N>,
    swapped: UnsafeCell<bool>,
}

impl<const N: usize> DoubleBuffer<N> {
    /// Constructs a new double-buffered primitive bump allocator.
    pub fn new() -> Self {
        DoubleBuffer {
            buffer_0: Buffer::new(),
            buffer_1: Buffer::new(),
            swapped: UnsafeCell::new(false),
        }
    }

    /// Empties both buffers invalidating previously allocated primitives.
    /// Allocated data remains intact until new allocations overwrite it.
    pub fn empty(&mut self) -> &mut Self {
        self.buffer_0.empty();
        self.buffer_1.empty();
        self
    }

    /// Swaps the currently selected buffer. This changes the result of
    /// dereferencing any `DoublePacket` allocated from this buffer.
    pub fn swap(&self) {
        unsafe {
            *self.swapped.get() = !*self.swapped.get();
        }
    }

    /// Allocate a double-buffered packet of type `T`. Returns `None` if
    /// remaining buffer space is insufficient.
    pub fn alloc<T: InitPrimitive>(&self) -> Option<DoublePacket<T>> {
        self.buffer_0
            .alloc::<T>()
            .map(move |data_0| {
                self.buffer_1.alloc::<T>().map(move |data_1| DoublePacket {
                    data_0,
                    data_1,
                    swapped: unsafe { &*self.swapped.get() },
                })
            })
            .flatten()
    }

    /// Allocates an array of double-buffered packets of type `T`. Returns
    /// `None` if remaining buffer space is insufficient.
    pub fn alloc_array<T: InitPrimitive, const M: usize>(&self) -> Option<[DoublePacket<T>; M]> {
        let mut ar: [DoublePacket<T>; M] = unsafe { MaybeUninit::zeroed().assume_init() };
        for i in 0..M {
            self.alloc().map(|p| ar[i] = p).or_else(|| return None);
        }
        Some(ar)
    }
}
