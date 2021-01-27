use crate::hal::dma::{BlockControl, ChannelControl, MemoryAddress};
use crate::hal::Mutable;
use crate::hal::{D0_BCR, D0_CHCR, D0_MADR};
use crate::hal::{D1_BCR, D1_CHCR, D1_MADR};
use crate::hal::{D2_BCR, D2_CHCR, D2_MADR};
use crate::hal::{D3_BCR, D3_CHCR, D3_MADR};
use crate::hal::{D4_BCR, D4_CHCR, D4_MADR};
use crate::hal::{D5_BCR, D5_CHCR, D5_MADR};
use crate::hal::{D6_BCR, D6_CHCR, D6_MADR};

mod channel;
mod gpu;

pub struct Channel<MADR: MemoryAddress, BCR: BlockControl, CHCR: ChannelControl> {
    madr: MADR,
    bcr: BCR,
    chcr: CHCR,
}

pub type MDECIn = Channel<D0_MADR<Mutable>, D0_BCR<Mutable>, D0_CHCR<Mutable>>;
pub type MDECOut = Channel<D1_MADR<Mutable>, D1_BCR<Mutable>, D1_CHCR<Mutable>>;
pub type GPU = Channel<D2_MADR<Mutable>, D2_BCR<Mutable>, D2_CHCR<Mutable>>;
pub type CDROM = Channel<D3_MADR<Mutable>, D3_BCR<Mutable>, D3_CHCR<Mutable>>;
pub type SPU = Channel<D4_MADR<Mutable>, D4_BCR<Mutable>, D4_CHCR<Mutable>>;
pub type PIO = Channel<D5_MADR<Mutable>, D5_BCR<Mutable>, D5_CHCR<Mutable>>;
pub type OTC = Channel<D6_MADR<Mutable>, D6_BCR<Mutable>, D6_CHCR<Mutable>>;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChannelName {
    MDECIn = 0,
    MDECOut,
    GPU,
    CDROM,
    SPU,
    PIO,
    OTC,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BlockMode {
    Single(u32),
    Multi { words: u16, blocks: u16 },
    LinkedList,
}

impl From<usize> for BlockMode {
    fn from(words: usize) -> BlockMode {
        BlockMode::Single(words as u32)
    }
}

impl From<u32> for BlockMode {
    fn from(words: u32) -> BlockMode {
        BlockMode::Single(words)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    ToMemory = 0,
    FromMemory,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Step {
    Forward = 0,
    Backward,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Chop {
    pub dma_win: u32,
    pub cpu_win: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TransferMode {
    Immediate = 0,
    Request,
    LinkedList,
}
