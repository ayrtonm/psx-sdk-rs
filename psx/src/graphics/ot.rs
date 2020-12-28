use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

use super::{InitPrimitive, LinkedList, TERMINATION};

use crate::graphics::packet::Packet;

/// An `N`-word ordering table.
pub struct OT<const N: usize> {
    entries: [u32; N],
}

impl Default for OT<1> {
    /// Creates an initialized 1-word ordering table.
    #[inline(always)]
    fn default() -> Self {
        OT {
            entries: [TERMINATION; 1],
        }
    }
}

impl<const N: usize> OT<N> {
    /// Creates an uninitialized ordering table.
    pub fn new() -> Self {
        OT { entries: [0; N] }
    }

    /// Gets the nth entry in an ordering table.
    pub fn entry(&self, n: usize) -> &u32 {
        &self.entries[n]
    }

    /// Inserts a packet into the nth slot in an ordering table.
    pub fn insert<'a, T: 'a + InitPrimitive, P>(
        &mut self, packet: &'a mut P, n: usize,
    ) -> &mut Self
    where &'a mut Packet<T>: From<&'a mut P> {
        let packet = <&mut Packet<T>>::from(packet);
        let tag = packet as *mut _ as *mut u32;
        unsafe {
            *tag &= !TERMINATION;
            *tag |= self.entries[n];
        }
        self.entries[n] = (tag as u32) & TERMINATION;
        self
    }
}

impl<const N: usize> LinkedList for OT<N> {
    fn start_address(&self) -> u32 {
        &self.entries[N - 1] as *const u32 as u32
    }
}

/// An `N`-entry double-buffered ordering table.
pub struct DoubleOT<const N: usize> {
    ot_0: OT<N>,
    ot_1: OT<N>,
    swapped: UnsafeCell<bool>,
}

impl Default for DoubleOT<1> {
    /// Creates an initialized double-buffered ordering table.
    #[inline(always)]
    fn default() -> Self {
        DoubleOT {
            ot_0: OT::default(),
            ot_1: OT::default(),
            swapped: UnsafeCell::new(false),
        }
    }
}

impl<const N: usize> DoubleOT<N> {
    /// Creates an uninitialized double-buffered ordering table.
    pub fn new() -> Self {
        DoubleOT {
            ot_0: OT::new(),
            ot_1: OT::new(),
            swapped: UnsafeCell::new(false),
        }
    }

    /// Swaps the currently selected ordering table.
    pub fn swap(&self) {
        unsafe {
            *self.swapped.get() = !*self.swapped.get();
        }
    }
}

impl<const N: usize> Deref for DoubleOT<N> {
    type Target = OT<N>;

    fn deref(&self) -> &Self::Target {
        if unsafe { *self.swapped.get() } {
            &self.ot_0
        } else {
            &self.ot_1
        }
    }
}

impl<const N: usize> DerefMut for DoubleOT<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if unsafe { *self.swapped.get() } {
            &mut self.ot_0
        } else {
            &mut self.ot_1
        }
    }
}

impl<const N: usize> LinkedList for DoubleOT<N> {
    fn start_address(&self) -> u32 {
        self.deref().start_address()
    }
}
