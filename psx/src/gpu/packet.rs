use crate::dma::LinkedList;
use crate::gpu::GPU_BUFFER_SIZE;
use crate::gpu::{Packet, PhysAddr};
use crate::hw::gpu::GP0Command;
use core::hint::black_box;
use core::mem::{size_of, transmute};
use core::slice;

impl<'a, T> From<&'a mut T> for PhysAddr {
    fn from(t: &'a mut T) -> PhysAddr {
        let ptr = t as *mut T as usize;
        PhysAddr([ptr as u8, (ptr >> 8) as u8, (ptr >> 16) as u8])
    }
}

const TERMINATION: PhysAddr = PhysAddr([0xFF, 0xFF, 0xFF]);

impl Packet<()> {
    /// Creates an empty [`Packet`]
    pub fn empty() -> Self {
        Packet {
            next: TERMINATION,
            size: 0,
            contents: (),
        }
    }
}

impl<T> Packet<T> {
    const SMALLER_THAN_BUFFER: () = {
        let size = size_of::<T>();
        if size > GPU_BUFFER_SIZE {
            panic!("Packet contents will overflow the GPU buffer. Use `Packet::new_unchecked` if this is intentional.");
        }
    };

    const SMALLER_THAN_U8_MAX: () = {
        let size = size_of::<T>();
        if size > u8::MAX as usize {
            panic!("Packet contents too large to be represented by `Packet` header.");
        }
    };

    /// Creates a new packet guaranteed to fit in the GPU buffer.
    #[allow(path_statements)]
    pub const fn new(t: T) -> Self {
        Self::SMALLER_THAN_BUFFER;
        let size = size_of::<T>() / size_of::<u32>();
        Packet {
            next: TERMINATION,
            size: size as u8,
            contents: t,
        }
    }

    /// Creates a new packet which may not fit into the GPU buffer.
    #[allow(path_statements)]
    pub const fn new_oversized(t: T) -> Self {
        Self::SMALLER_THAN_U8_MAX;
        let size = size_of::<T>() / size_of::<u32>();
        Packet {
            next: TERMINATION,
            size: size as u8,
            contents: t,
        }
    }

    /// Gets a reference to the [`Packet`] header.
    fn header_address(&self) -> &u32 {
        let ptr = self.next.0.as_ptr() as *const u32;
        // TODO: The first word should be a union or u32 to avoid UB
        unsafe { ptr.as_ref().unwrap() }
    }

    /// Gets the [`Packet`] header.
    pub fn header(&self) -> u32 {
        let res = [self.next.0[0], self.next.0[1], self.next.0[2], self.size];
        u32::from_le_bytes(res)
    }

    /// Inserts `other` between `self` and the following packet.
    ///
    /// before: `self` -> `next`
    /// after: `self` -> `other` -> `next`
    ///
    /// Note that `self` may be the last [`Packet`] in which case `next` does
    /// not exist. Returns the `PhysAddr` `other` previously pointed to, if any.
    pub fn insert_packet<U>(&mut self, other: &mut Packet<U>) -> Option<PhysAddr> {
        // FIXME: This is a complete hack since black_box should not be relied on for
        // correctness.
        let other = black_box(other);
        let res = other.next;
        other.next = self.next;
        self.next = PhysAddr::from(other);
        if res == TERMINATION {
            None
        } else {
            Some(res)
        }
    }

    /// Inserts `other` between `self` and the following packet.
    ///
    /// before: `self` -> `next`
    /// after: `self` -> `other.first` -> ... -> `other.last` -> `next`
    ///
    /// Note that `self` may be the last [`Packet`] in which case `next` does
    /// not exist. Returns the `PhysAddr` `other` previously pointed to, if any.
    pub fn insert_list<U>(&mut self, other: &mut [Packet<U>]) -> Option<PhysAddr> {
        // FIXME: This is a complete hack since black_box should not be relied on for
        // correctness.
        let other = black_box(other);
        let last = other.last_mut()?;
        let res = last.next;
        last.next = self.next;
        self.next = PhysAddr::from(other.first_mut()?);
        if res == TERMINATION {
            None
        } else {
            Some(res)
        }
    }
}

/// Initializes an ordering table from a `&mut [u32]`.
///
/// list\[0\] -> list\[1\] -> list\[2\] -> ... -> list\[n\]
///
/// Note the packets are linked from first to last.
pub fn ordering_table<T>(list: &mut [u32]) -> &mut [Packet<()>] {
    let n = list.len();
    let packets = unsafe { transmute::<&mut [u32], &mut [Packet<()>]>(list) };
    for i in 0..n {
        packets[i].size = 0;
    }
    link_list(packets);
    unsafe { slice::from_raw_parts_mut(list.as_mut_ptr() as *mut Packet<()>, n) }
}

/// Link an existing array of packets from first to last.
///
/// list\[0\] -> list\[1\] -> list\[2\] -> ... -> list\[n\]
pub fn link_list<T>(list: &mut [Packet<T>]) {
    let n = list.len();
    for i in 1..n {
        let (a, b) = list.split_at_mut(i);
        let last_a = &mut a[a.len() - 1];
        let first_b = &mut b[0];
        last_a.insert_packet(first_b);
    }
}

impl<T> LinkedList for Packet<T>
where T: GP0Command
{
    fn address(&self) -> Option<&u32> {
        Some(self.header_address())
    }
}
impl LinkedList for Packet<()> {
    fn address(&self) -> Option<&u32> {
        Some(self.header_address())
    }
}
impl<T> LinkedList for [Packet<T>]
where T: GP0Command
{
    fn address(&self) -> Option<&u32> {
        self.first().map(|p| p.header_address())
    }
}
impl LinkedList for [Packet<()>] {
    fn address(&self) -> Option<&u32> {
        self.first().map(|p| p.header_address())
    }
}
impl<const N: usize> LinkedList for [Packet<()>; N] {
    fn address(&self) -> Option<&u32> {
        self.first().map(|p| p.header_address())
    }
}
impl<T, const N: usize> LinkedList for [Packet<T>; N]
where T: GP0Command
{
    fn address(&self) -> Option<&u32> {
        self.first().map(|p| p.header_address())
    }
}
