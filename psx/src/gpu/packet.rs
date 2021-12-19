use crate::dma::LinkedList;
use crate::hw::gpu::GP0Command;
use crate::Result;
use core::convert::TryFrom;
use core::mem::size_of;

#[derive(Debug)]
pub struct PhysAddr([u8; 3]);

const TERMINATION: [u8; 3] = [0xFF, 0xFF, 0xFF];

#[repr(C)]
#[derive(Debug)]
pub struct Packet<T> {
    next: PhysAddr,
    size: u8,
    pub payload: T,
}

impl<T> Packet<T> {
    pub fn new(t: T) -> Result<Self> {
        let bytes = u8::try_from(size_of::<T>()).map_err(|_| "")?;
        Ok(Packet {
            next: PhysAddr(TERMINATION),
            size: bytes / 4,
            payload: t,
        })
    }
}

impl<T> LinkedList for Packet<T> where T: GP0Command {}
