use crate::hal::{MutRegister, Mutable, Register, State};
use crate::hal::{D0_BCR, D0_CHCR, D0_MADR};
use crate::hal::{D1_BCR, D1_CHCR, D1_MADR};
use crate::hal::{D2_BCR, D2_CHCR, D2_MADR};
use crate::hal::{D3_BCR, D3_CHCR, D3_MADR};
use crate::hal::{D4_BCR, D4_CHCR, D4_MADR};
use crate::hal::{D5_BCR, D5_CHCR, D5_MADR};
use crate::hal::{D6_BCR, D6_CHCR, D6_MADR};
use crate::illegal;

#[macro_use]
mod channels;
mod control;

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

const STEP: u32 = 1;
const CHOP: u32 = 8;
const TRANSFER_MODE: u32 = 9;
const DMA_WIN: u32 = 16;
const CPU_WIN: u32 = 20;
const BUSY: u32 = 24;
const TRIGGER: u32 = 28;

pub trait MemoryAddress: Register<u32> {}

pub trait MutMemoryAddress: MutRegister<u32> + MemoryAddress {}

pub trait BlockControl: Register<u32> {
    fn get_block(&self, transfer_mode: TransferMode) -> Option<BlockMode> {
        match transfer_mode {
            TransferMode::Immediate => match self.get() {
                0 => Some(0x1_0000u32.into()),
                1..=0xFFFF => Some(self.get().into()),
                _ => None,
            },
            TransferMode::Request => Some(BlockMode::Multi {
                words: self.get() as u16,
                blocks: (self.get() >> 16) as u16,
            }),
            TransferMode::LinkedList => Some(BlockMode::LinkedList),
        }
    }
}

pub trait MutBlockControl: MutRegister<u32> + BlockControl {
    fn set_block<B: Into<BlockMode>>(&mut self, block_mode: B) -> &mut Self {
        match block_mode.into() {
            BlockMode::Single(words) => {
                let words = match words {
                    0..=0xFFFF => words as u32,
                    0x1_0000 => 0,
                    _ => illegal(),
                };
                *self.get_mut() = words;
            },
            BlockMode::Multi { words, blocks } => {
                let value = words as u32 | ((blocks as u32) << 16);
                *self.get_mut() = value;
            },
            BlockMode::LinkedList => (),
        };
        self
    }
}

pub trait ChannelControl: Register<u32> {
    fn wait(&mut self) {
        while self.contains(1 << BUSY) {
            self.reload();
        }
    }
    fn get_mode(&self) -> Option<TransferMode> {
        match (self.get() >> TRANSFER_MODE) & 0b11 {
            0 => Some(TransferMode::Immediate),
            1 => Some(TransferMode::Request),
            2 => Some(TransferMode::LinkedList),
            _ => None,
        }
    }

    fn busy(&self) -> bool {
        self.contains(1 << BUSY)
    }

    fn get_direction(&self) -> Direction {
        if self.contains(1) {
            Direction::FromMemory
        } else {
            Direction::ToMemory
        }
    }

    fn get_step(&self) -> Step {
        if self.contains(1 << STEP) {
            Step::Backward
        } else {
            Step::Forward
        }
    }

    fn get_chop(&self) -> Option<Chop> {
        if self.contains(1 << CHOP) {
            Some(Chop {
                cpu_win: (self.get() >> CPU_WIN) & 0b111,
                dma_win: (self.get() >> DMA_WIN) & 0b111,
            })
        } else {
            None
        }
    }
}

pub trait MutChannelControl: MutRegister<u32> + ChannelControl {
    fn set_direction(&mut self, direction: Direction) -> &mut Self {
        self.clear(1).set(direction as u32)
    }

    fn set_step(&mut self, step: Step) -> &mut Self {
        self.clear(1 << STEP).set((step as u32) << STEP)
    }

    fn set_chop(&mut self, chop: Option<Chop>) -> &mut Self {
        match chop {
            Some(chop) => self
                .clear(chop.cpu_win << CPU_WIN | chop.dma_win << DMA_WIN)
                .set(1 << CHOP | chop.cpu_win << CPU_WIN | chop.dma_win << DMA_WIN),
            None => self.clear(1 << CHOP),
        }
    }

    fn set_mode(&mut self, mode: TransferMode) -> &mut Self {
        self.clear(0b11 << TRANSFER_MODE)
            .set((mode as u32) << TRANSFER_MODE)
    }

    fn start(&mut self) -> &mut Self {
        if let Some(TransferMode::Immediate) = self.get_mode() {
            self.set(1 << TRIGGER);
        }
        self.set(1 << BUSY)
    }

    fn stop(&mut self) -> &mut Self {
        self.clear(1 << BUSY)
    }
}

channel! {
    [D0_MADR, D0_BCR, D0_CHCR],
    [D1_MADR, D1_BCR, D1_CHCR],
    [D2_MADR, D2_BCR, D2_CHCR],
    [D3_MADR, D3_BCR, D3_CHCR],
    [D4_MADR, D4_BCR, D4_CHCR],
    [D5_MADR, D5_BCR, D5_CHCR],
    [D6_MADR, D6_BCR, D6_CHCR]
}
