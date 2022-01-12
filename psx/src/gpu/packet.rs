use crate::dma::LinkedList;
use crate::gpu::{Packet, PacketError, PhysAddr};
use crate::hw::gpu::GP0Command;
use crate::KSEG0;
use core::mem::{forget, size_of, transmute};
use core::slice;

impl<'a, T> From<&'a mut T> for PhysAddr {
    fn from(t: &'a mut T) -> PhysAddr {
        let ptr = t as *mut T as usize;
        PhysAddr([ptr as u8, (ptr >> 8) as u8, (ptr >> 16) as u8])
    }
}

impl<T> From<PhysAddr> for *mut Packet<T> {
    fn from(phys_addr: PhysAddr) -> *mut Packet<T> {
        let addr = phys_addr.0;
        let ptr = addr[0] as usize | ((addr[1] as usize) << 8) | ((addr[2] as usize) << 16) | KSEG0;
        ptr as *mut Packet<T>
    }
}

const TERMINATION: PhysAddr = PhysAddr([0xFF, 0xFF, 0xFF]);

impl Packet<()> {
    pub fn empty() -> Self {
        Packet {
            next: TERMINATION,
            size: 0,
            contents: (),
        }
    }
}

// The GPU buffer can only fit 64 bytes.
const BUFFER_SIZE: usize = 64;

impl<T> Packet<T> {
    const VALIDATE_SIZE: () = {
        let size = size_of::<T>();
        if size > BUFFER_SIZE {
            panic!("Packet contents will overflow the GPU buffer. Use `Packet::new_unchecked` if this is intentional.");
        }
    };
    /// Creates a new packet guaranteed to fit in the GPU buffer.
    pub const fn new(t: T) -> Self {
        Self::VALIDATE_SIZE;
        let size = size_of::<T>() / size_of::<u32>();
        Packet {
            next: TERMINATION,
            size: size as u8,
            contents: t,
        }
    }

    /// Creates a new packet which may not fit into the GPU buffer.
    pub const fn new_unchecked(t: T) -> Result<Self, PacketError> {
        let bytes = size_of::<T>();
        if bytes > u8::MAX as usize {
            // Prevent `t`'s destructors from running to allow making this a const fn
            forget(t);
            return Err(PacketError::Oversized)
        }
        let bytes = bytes as u8;
        Ok(Packet {
            next: TERMINATION,
            size: bytes / 4,
            contents: t,
        })
    }

    pub fn tag(&self) -> u32 {
        let res = [self.next.0[0], self.next.0[1], self.next.0[2], self.size];
        u32::from_le_bytes(res)
    }

    /// Inserts `other` between `self` and the packet it points to. Returns the
    /// PhysAddr `other` previously pointed to.
    pub fn insert_packet<U>(&mut self, other: &mut Packet<U>) -> Option<PhysAddr> {
        let res = other.next;
        other.next = self.next;
        self.next = PhysAddr::from(other);
        if res == TERMINATION {
            None
        } else {
            Some(res)
        }
    }

    /// Inserts `other` between `self` and the packet it points to. Returns the
    /// PhysAddr `other` previously pointed to.
    pub fn insert_list<U>(&mut self, other: &mut [Packet<U>]) -> Option<PhysAddr> {
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

/// Initializes an ordering table from a `&mut [u32]`. Note the packets are
/// linked from first to last.
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
pub fn link_list<T>(list: &mut [Packet<T>]) {
    let n = list.len();
    for i in 1..n {
        let (a, b) = list.split_at_mut(i);
        let last_a = &mut a[a.len() - 1];
        let first_b = &mut b[0];
        last_a.insert_packet(first_b);
    }
}

impl<T> LinkedList for Packet<T> where T: GP0Command {}
impl LinkedList for Packet<()> {}
impl<T> LinkedList for [Packet<T>] where T: GP0Command {}
impl LinkedList for [Packet<()>] {}
