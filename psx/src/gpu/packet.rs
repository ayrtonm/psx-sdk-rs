use crate::dma::LinkedList;
use crate::gpu::{OrderingTable, Packet, PhysAddr};
use crate::hw::gpu::GP0Command;
use core::convert::TryFrom;
use core::mem::size_of;
use core::mem::MaybeUninit;
use strum_macros::IntoStaticStr;

impl<'a, T> From<&'a mut T> for PhysAddr {
    fn from(t: &'a mut T) -> PhysAddr {
        let ptr = t as *mut T as usize;
        PhysAddr([ptr as u8, (ptr >> 8) as u8, (ptr >> 16) as u8])
    }
}

const TERMINATION: PhysAddr = PhysAddr([0xFF, 0xFF, 0xFF]);

// The GPU buffer can only fit 64 bytes.
const BUFFER_SIZE: usize = 64;

impl Packet<()> {
    pub fn empty() -> Self {
        Packet {
            next: TERMINATION,
            size: 0,
            payload: (),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoStaticStr)]
pub enum Error {
    BufferOverflow,
    Oversized,
}

impl<T> Packet<T> {
    /// Creates a new packet guaranteed to fit in the GPU buffer.
    pub fn new(t: T) -> Result<Self, Error> {
        if size_of::<T>() > BUFFER_SIZE {
            Err(Error::BufferOverflow)
        } else {
            Self::new_oversized(t)
        }
    }

    /// Creates a new packet which may not fit into the GPU buffer.
    pub fn new_oversized(t: T) -> Result<Self, Error> {
        let bytes = u8::try_from(size_of::<T>()).map_err(|_| Error::Oversized)?;
        Ok(Packet {
            next: TERMINATION,
            size: bytes / 4,
            payload: t,
        })
    }

    pub fn new_array<const N: usize>(ts: [T; N]) -> Result<[Self; N], Error> {
        let mut array = unsafe { MaybeUninit::<[Self; N]>::zeroed().assume_init() };
        for (i, t) in ts.into_iter().enumerate() {
            array[i] = Packet::new(t)?;
        }
        Ok(array)
    }

    pub fn tag(&self) -> u32 {
        let res = [self.next.0[0], self.next.0[1], self.next.0[2], self.size];
        u32::from_le_bytes(res)
    }

    pub fn link(&mut self, other: &mut Self) -> Option<PhysAddr> {
        let res = other.next;
        other.next = self.next;
        self.next = PhysAddr::from(other);
        if res == TERMINATION {
            None
        } else {
            Some(res)
        }
    }
}

impl<T, const N: usize> OrderingTable<T, N> {
    pub fn new(ts: [T; N]) -> Result<OrderingTable<T, N>, Error> {
        Ok(OrderingTable {
            list: Packet::new_array(ts)?,
        })
    }

    pub fn link(&mut self) {
        for i in 1..N {
            let (a, b) = self.list.split_at_mut(i);
            let last_a = &mut a[a.len() - 1];
            let first_b = &mut b[0];
            last_a.link(first_b);
        }
    }
}

impl<T> LinkedList for Packet<T> where T: GP0Command {}
impl LinkedList for Packet<()> {}
impl<T, const N: usize> LinkedList for OrderingTable<T, N> where T: GP0Command {}
