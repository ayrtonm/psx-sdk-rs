use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

use super::{Init, Packet};

/// A depth [ordering table](http://problemkaputt.de/psx-spx.htm#gpudepthordering)
pub struct OT<const N: usize> {
    entries: [u32; N],
}

impl<const N: usize> OT<N> {
    pub fn new() -> Self {
        OT { entries: [0; N] }
    }

    pub fn start(&self) -> usize {
        N - 1
    }

    pub fn first_entry(&self) -> &u32 {
        &self.entries[N - 1]
    }

    pub fn entry(&self, n: usize) -> &u32 {
        &self.entries[n]
    }

    // TODO: combine `insert` and `add_prim` into one function
    pub fn insert<T: Init, U>(&mut self, prim: &mut U, z: usize) -> &mut Self
    where U: Deref<Target = Packet<T>> + DerefMut {
        self.add_prim(prim, z)
    }

    pub fn add_prim<'a, T: 'a + Init, U: 'a>(&mut self, prim: &'a mut U, z: usize) -> &mut Self
    where &'a mut Packet<T>: From<&'a mut U> {
        let prim = <&'a mut Packet<T> as From<&'a mut U>>::from(prim);
        let tag = prim as *mut _ as *mut u32;
        unsafe {
            *tag &= !0x00FF_FFFF;
            *tag |= self.entries[z];
            self.entries[z] = (tag as u32) & 0x00FF_FFFF;
        }
        self
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
