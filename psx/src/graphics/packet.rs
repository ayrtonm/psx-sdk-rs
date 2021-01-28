use super::{num_words, TERMINATION};
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

#[repr(C)]
pub struct Packet<T> {
    tag: u32,
    data: T,
}

impl<T> Packet<T> {
    pub const fn new(data: T, size: Option<u8>) -> Packet<T> {
        let default_size = num_words::<T>();
        let size = match size {
            Some(size) => size as u32,
            None => default_size as u32,
        };
        Packet {
            tag: (size << 24) | TERMINATION,
            data,
        }
    }

    pub fn reset(&mut self) {
        let size = num_words::<T>() << 24;
        self.tag = size as u32 | TERMINATION;
    }
}

impl<T> Deref for Packet<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Packet<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

pub struct Ref<'a, T>(&'a mut Packet<T>);

impl<'a, T> Ref<'a, T> {
    pub fn new(packet_ref: &'a mut Packet<T>) -> Self {
        Ref(packet_ref)
    }
}
impl<'a, T> Deref for Ref<'a, T> {
    type Target = Packet<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> DerefMut for Ref<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct DoubleRef<'a, T> {
    data_0: UnsafeCell<Ref<'a, T>>,
    data_1: UnsafeCell<Ref<'a, T>>,
    swapped: &'a bool,
}

impl<'a, T> DoubleRef<'a, T> {
    pub const fn new(data_0: Ref<'a, T>, data_1: Ref<'a, T>, swapped: &'a bool) -> Self {
        DoubleRef {
            data_0: UnsafeCell::new(data_0),
            data_1: UnsafeCell::new(data_1),
            swapped,
        }
    }

    pub fn split(&mut self) -> (&mut Ref<'a, T>, &mut Ref<'a, T>) {
        if *self.swapped {
            (self.data_0.get_mut(), self.data_1.get_mut())
        } else {
            (self.data_1.get_mut(), self.data_0.get_mut())
        }
    }
}

impl<'a, T> Deref for DoubleRef<'a, T> {
    type Target = Ref<'a, T>;

    fn deref(&self) -> &Self::Target {
        if *self.swapped {
            unsafe { &*self.data_0.get() }
        } else {
            unsafe { &*self.data_1.get() }
        }
    }
}

impl<'a, T> DerefMut for DoubleRef<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if *self.swapped {
            self.data_0.get_mut()
        } else {
            self.data_1.get_mut()
        }
    }
}
