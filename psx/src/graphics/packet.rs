use core::mem::size_of;
use core::ops::{Deref, DerefMut};

use super::{LinkedList, TERMINATION};

/// A wrapper-type for inserting arbitrary data into an ordering table.
#[repr(C)]
pub struct Packet<T> {
    pub(crate) tag: u32,
    pub(crate) data: T,
}

impl<T> Packet<T> {
    /// Creates a new standalone packet.
    pub fn new(data: T, size: Option<u32>) -> Packet<T> {
        let size = size.unwrap_or((size_of::<T>() / 4) as u32) << 24;
        Packet {
            tag: size | TERMINATION,
            data,
        }
    }

    /// Resets a packet's tag separating it from any linked list it belongs to.
    pub fn reset(&mut self) {
        let size = ((size_of::<T>() / 4) as u32) << 24;
        self.tag = size | TERMINATION;
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

// If something can be deref'ed to a Packet (e.g. DoublePacket), it should be
// possible to convert &mut P to &mut Packet as well. Used in OT::insert.
impl<'a, T, P> From<&'a mut P> for &'a mut Packet<T>
where P: Deref<Target = Packet<T>> + DerefMut
{
    fn from(p: &'a mut P) -> &'a mut Packet<T> {
        p
    }
}

impl<T> LinkedList for Packet<T> {
    fn start_address(&self) -> &u32 {
        &self.tag
    }
}

/// A packet created from a double-buffered primivite bump allocator. Method
/// calls only affect the primitive from the currently selected buffer in the
/// source allocator.
pub struct DoublePacket<'a, T> {
    pub(crate) data_0: &'a mut Packet<T>,
    pub(crate) data_1: &'a mut Packet<T>,
    pub(crate) swapped: &'a bool,
}

impl<T> DoublePacket<'_, T> {
    /// Gets mutable references to the current and alternative packets
    /// respectively.
    pub fn split(&mut self) -> (&mut Packet<T>, &mut Packet<T>) {
        if *self.swapped {
            (self.data_0, self.data_1)
        } else {
            (self.data_1, self.data_0)
        }
    }
}

impl<'a, T> Deref for DoublePacket<'a, T> {
    type Target = Packet<T>;

    fn deref(&self) -> &Self::Target {
        if *self.swapped {
            &self.data_0
        } else {
            &self.data_1
        }
    }
}

impl<'a, T> DerefMut for DoublePacket<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if *self.swapped {
            &mut self.data_0
        } else {
            &mut self.data_1
        }
    }
}

impl<T> LinkedList for DoublePacket<'_, T> {
    fn start_address(&self) -> &u32 {
        self.deref().start_address()
    }
}
