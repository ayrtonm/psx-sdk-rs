use crate::dma::LinkedList;
use crate::hw::gpu::GP0Command;
use core::convert::TryFrom;
use core::mem::{size_of, transmute};
use strum_macros::IntoStaticStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysAddr([u8; 3]);

impl<'a, T> From<&'a T> for PhysAddr {
    fn from(t: &T) -> PhysAddr {
        let ptr = t as *const T as usize;
        PhysAddr([ptr as u8, (ptr >> 8) as u8, (ptr >> 16) as u8])
    }
}

impl<'a, T> From<&'a mut T> for PhysAddr {
    fn from(t: &mut T) -> PhysAddr {
        let ptr = t as *const T as usize;
        PhysAddr([ptr as u8, (ptr >> 8) as u8, (ptr >> 16) as u8])
    }
}

const TERMINATION: PhysAddr = PhysAddr([0xFF, 0xFF, 0xFF]);

#[repr(C)]
#[derive(Debug)]
pub struct Packet<T> {
    next: PhysAddr,
    size: u8,
    pub payload: T,
}

impl Packet<()> {
    pub fn empty() -> Self {
        Packet {
            next: TERMINATION,
            size: 0,
            payload: (),
        }
    }

    pub fn init_table<const N: usize>(buf: &mut [u32; N]) -> &mut [Self; N] {
        let mut res: [Self; N] = buf.map(|tag| unsafe { transmute(tag) });
        res[0].next = TERMINATION;
        for i in 1..N {
            res[i].next = PhysAddr::from(&res[i - 1]);
        }
        unsafe { transmute(buf) }
        //let mut res: [Self; N] = buf.map(|tag| unsafe { transmute(tag) });
        //res[0].next = TERMINATION;
        //for i in 1..N {
        //    res[i].next = PhysAddr::from(&res[i - 1])
        //}
        //res
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoStaticStr)]
pub enum Error {
    OversizedPayload,
}

impl<T> Packet<T> {
    pub fn new(t: T) -> Result<Self, Error> {
        let bytes = u8::try_from(size_of::<T>()).map_err(|_| Error::OversizedPayload)?;
        Ok(Packet {
            next: TERMINATION,
            size: bytes / 4,
            payload: t,
        })
    }

    pub fn tag(&self) -> u32 {
        let res = [self.next.0[0], self.next.0[1], self.next.0[2], self.size];
        u32::from_le_bytes(res)
    }

    pub fn insert(&mut self, other: &mut Self) -> PhysAddr {
        let res = other.next;
        other.next = self.next;
        self.next = PhysAddr::from(other);
        res
    }
}

impl<T> LinkedList for Packet<T> where T: GP0Command {}
impl LinkedList for Packet<()> {}
