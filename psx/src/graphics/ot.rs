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
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    fn default() -> Self {
        OT {
            entries: [TERMINATION; 1],
        }
    }
}

impl OT<1> {
    /// Empties the ordering table by storing the termination code in the entry.
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    pub fn empty(&mut self) -> &mut Self {
        self.entries[0] = TERMINATION;
        self
    }
}

impl<const N: usize> OT<N> {
    /// Creates an uninitialized ordering table.
    pub fn new() -> Self {
        OT { entries: [0; N] }
    }

    /// Gets the nth entry in an ordering table.
    pub fn entry(&self, n: usize) -> &u32 {
        unsafe { self.entries.get_unchecked(n) }
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
            *tag |= *self.entries.get_unchecked(n);
            *self.entries.get_unchecked_mut(n) = (tag as u32) & TERMINATION;
        }
        self
    }
}

impl<const N: usize> LinkedList for OT<N> {
    fn start_address(&self) -> &u32 {
        &self.entries[N - 1]
    }
}

/// An `N`-entry double-buffered ordering table.
pub struct DoubleOT<const N: usize> {
    ot_0: UnsafeCell<OT<N>>,
    ot_1: UnsafeCell<OT<N>>,
    swapped: UnsafeCell<bool>,
}

impl Default for DoubleOT<1> {
    /// Creates an initialized double-buffered ordering table.
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    fn default() -> Self {
        DoubleOT {
            ot_0: UnsafeCell::new(OT::default()),
            ot_1: UnsafeCell::new(OT::default()),
            swapped: UnsafeCell::new(false),
        }
    }
}

//impl DoubleOT<1> {
//    /// Empties the current ordering table by storing the termination code in
//    /// its entry.
//    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
//    pub fn empty(&self) -> &Self {
//        self.unsafe_deref().empty();
//        self
//    }
//}

impl<const N: usize> DoubleOT<N> {
    /// Creates an uninitialized double-buffered ordering table.
    pub fn new() -> Self {
        DoubleOT {
            ot_0: UnsafeCell::new(OT::new()),
            ot_1: UnsafeCell::new(OT::new()),
            swapped: UnsafeCell::new(false),
        }
    }

    /// Swaps the currently selected ordering table.
    pub fn swap(&self) -> &Self {
        unsafe {
            *self.swapped.get() = !*self.swapped.get();
        }
        self
    }

    /*
    /// Gets a mutable reference to the current ordering table.
    pub unsafe fn get_mut(&self) -> &mut OT<N> {
        if *self.swapped.get() {
            &mut *self.ot_0.get()
        } else {
            &mut *self.ot_1.get()
        }
    }

    /// Gets a mutable reference to the alternate ordering table.
    pub unsafe fn alt_get_mut(&self) -> &mut OT<N> {
        if !*self.swapped.get() {
            &mut *self.ot_0.get()
        } else {
            &mut *self.ot_1.get()
        }
    }

    /// Returns mutable references to the current and alternate ordering
    /// tables respectively.
    pub fn split(&self) -> (&mut OT<N>, &mut OT<N>) {
        unsafe {
            let current = self.get_mut();
            let alt = self.alt_get_mut();
            (current, alt)
        }
    }
    */
}

impl<const N: usize> Deref for DoubleOT<N> {
    type Target = OT<N>;

    fn deref(&self) -> &Self::Target {
        // Not actually unsafe since the *mut OT<N> are cast to &OT<N>
        unsafe {
            if *self.swapped.get() {
                &*self.ot_0.get()
            } else {
                &*self.ot_1.get()
            }
        }
    }
}

impl<const N: usize> DerefMut for DoubleOT<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if *self.swapped.get_mut() {
            self.ot_0.get_mut()
        } else {
            self.ot_1.get_mut()
        }
    }
}

impl<const N: usize> LinkedList for DoubleOT<N> {
    fn start_address(&self) -> &u32 {
        self.deref().start_address()
    }
}
