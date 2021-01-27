use super::{DoubleRef, Initialize, Packet, Ref};
use crate::num_words;
use core::cell::UnsafeCell;
use core::mem::MaybeUninit;

pub struct Buffer<const N: usize>(UnsafeCell<InnerBuffer<N>>);

pub struct DoubleBuffer<const N: usize> {
    buffer_0: Buffer<N>,
    buffer_1: Buffer<N>,
    swapped: UnsafeCell<bool>,
}

struct InnerBuffer<const N: usize> {
    data: [u32; N],
    next: usize,
}

impl<const N: usize> Buffer<N> {
    pub const fn new() -> Self {
        Buffer(UnsafeCell::new(InnerBuffer {
            data: [0; N],
            next: 0,
        }))
    }

    pub fn empty(&mut self) -> &mut Self {
        self.0.get_mut().next = 0;
        self
    }

    pub fn words_remaining(&self) -> usize {
        unsafe { N - (*self.0.get()).next }
    }

    pub fn packet<T: Initialize>(&self) -> Option<Ref<T>> {
        self.allocate::<Packet<T>>().map(|p| {
            p.reset();
            p.init();
            Ref::new(p)
        })
    }

    pub fn packet_array<T: Initialize, const M: usize>(&self) -> Option<[Ref<T>; M]> {
        let mut ar: [Ref<T>; M] = unsafe { MaybeUninit::zeroed().assume_init() };
        for i in 0..M {
            self.packet().map(|p| ar[i] = p).or_else(|| return None);
        }
        Some(ar)
    }

    fn allocate<T>(&self) -> Option<&mut T> {
        let words = num_words::<T>();
        let start = unsafe { (*self.0.get()).next };
        let end = start + words;
        if end <= N {
            unsafe {
                (*self.0.get()).next += words;
                let slice = &mut (*self.0.get()).data.get_unchecked_mut(start..end);
                let ptr = slice.as_mut_ptr().cast::<T>();
                ptr.as_mut()
            }
        } else {
            None
        }
    }
}

impl<const N: usize> DoubleBuffer<N> {
    pub const fn new() -> Self {
        DoubleBuffer {
            buffer_0: Buffer::new(),
            buffer_1: Buffer::new(),
            swapped: UnsafeCell::new(false),
        }
    }

    pub fn swap(&self) {
        unsafe {
            *self.swapped.get() = !*self.swapped.get();
        }
    }

    pub fn packet<T: Initialize>(&self) -> Option<DoubleRef<T>> {
        self.buffer_0
            .packet::<T>()
            .map(move |data_0| {
                self.buffer_1.packet::<T>().map(move |data_1| {
                    DoubleRef::new(data_0, data_1, unsafe { &*self.swapped.get() })
                })
            })
            .flatten()
    }

    pub fn packet_array<T: Initialize, const M: usize>(&self) -> Option<[DoubleRef<T>; M]> {
        let mut ar: [DoubleRef<T>; M] = unsafe { MaybeUninit::zeroed().assume_init() };
        for i in 0..M {
            self.packet().map(|p| ar[i] = p).or_else(|| return None);
        }
        Some(ar)
    }
}
