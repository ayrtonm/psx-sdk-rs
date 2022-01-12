use crate::dma::LinkedList;
use crate::gpu::{Packet, PhysAddr};
use crate::hw::gpu::GP0Command;
use core::mem::{forget, size_of, MaybeUninit};
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
            contents: (),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoStaticStr)]
pub enum Error {
    Oversized,
}

const A: Packet<[u8; BUFFER_SIZE]> = Packet::new([0; BUFFER_SIZE]);
const B: Result<Packet<[u8; BUFFER_SIZE + 1]>, Error> = Packet::new_unchecked([0; BUFFER_SIZE + 1]);

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
    pub const fn new_unchecked(t: T) -> Result<Self, Error> {
        let bytes = size_of::<T>();
        if bytes > u8::MAX as usize {
            // Prevent `t`'s destructors from running to allow making this a const fn
            forget(t);
            return Err(Error::Oversized)
        }
        let bytes = bytes as u8;
        Ok(Packet {
            next: TERMINATION,
            size: bytes / 4,
            contents: t,
        })
    }

    pub fn new_array<const N: usize>(ts: [T; N]) -> [Self; N] {
        let mut array = unsafe { MaybeUninit::<[Self; N]>::zeroed().assume_init() };
        for (i, t) in ts.into_iter().enumerate() {
            array[i] = Packet::new(t);
        }
        array
    }

    pub fn tag(&self) -> u32 {
        let res = [self.next.0[0], self.next.0[1], self.next.0[2], self.size];
        u32::from_le_bytes(res)
    }

    pub fn link<U>(&mut self, other: &mut [Packet<U>]) -> Option<PhysAddr> {
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

//impl<T, const N: usize> OrderingTable<T, N> {
//    pub fn new(ts: [T; N]) -> Result<OrderingTable<T, N>, Error> {
//        Ok(OrderingTable {
//            list: Packet::new_array(ts)?,
//        })
//    }
//
pub fn link<T>(list: &mut [Packet<T>]) {
    let n = list.len();
    for i in 1..n {
        let (a, b) = list.split_at_mut(i);
        let last_a = &mut a[a.len() - 1];
        let (first_b, _) = b.split_at_mut(1);
        last_a.link(first_b);
    }
}
//}

//impl<T, const N: usize> LinkedList for OrderingTable<T, N> where T:
// GP0Command {}
impl<T> LinkedList for Packet<T> where T: GP0Command {}
impl LinkedList for Packet<()> {}
impl<T> LinkedList for [Packet<T>] where T: GP0Command {}
impl LinkedList for [Packet<()>] {}
