use super::ty::{BlockMode, Chop, Direction, Step, TransferMode};
use crate::hal::{MutRegister, Register};

const STEP: u32 = 1;
const CHOP: u32 = 8;
const TRANSFER_MODE: u32 = 9;
const DMA_WIN: u32 = 16;
const CPU_WIN: u32 = 20;
const BUSY: u32 = 24;
const TRIGGER: u32 = 28;

pub trait SharedMemoryAddress: Register<u32> {
    fn get_address(&self) -> *const u32 {
        self.get() as *const u32
    }
}

pub trait MemoryAddress: MutRegister<u32> + SharedMemoryAddress {
    fn set_address(&mut self, ptr: *const u32) -> &mut Self {
        self.assign(ptr as u32)
    }
}

pub trait SharedBlockControl: Register<u32> {
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

pub trait BlockControl: MutRegister<u32> + SharedBlockControl {
    fn set_block<B: Into<BlockMode>>(&mut self, block_mode: B) -> &mut Self {
        match block_mode.into() {
            BlockMode::Single(words) => {
                let words = match words {
                    0..=0xFFFF => words as u32,
                    0x1_0000 => 0,
                    _ => illegal!("TODO\0"),
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

pub trait SharedChannelControl: Register<u32> {
    fn wait(&mut self) {
        while self.busy() {
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
        self.all_set(1 << BUSY)
    }

    fn get_direction(&self) -> Direction {
        if self.all_set(1) {
            Direction::FromMemory
        } else {
            Direction::ToMemory
        }
    }

    fn get_step(&self) -> Step {
        if self.all_set(1 << STEP) {
            Step::Backward
        } else {
            Step::Forward
        }
    }

    fn get_chop(&self) -> Option<Chop> {
        if self.all_set(1 << CHOP) {
            Some(Chop {
                cpu_win: (self.get() >> CPU_WIN) & 0b111,
                dma_win: (self.get() >> DMA_WIN) & 0b111,
            })
        } else {
            None
        }
    }
}

pub trait ChannelControl: MutRegister<u32> + SharedChannelControl {
    fn set_direction(&mut self, direction: Direction) -> &mut Self {
        self.clear_bits(1).set_bits(direction as u32)
    }

    fn set_step(&mut self, step: Step) -> &mut Self {
        self.clear_bits(1 << STEP).set_bits((step as u32) << STEP)
    }

    fn set_chop(&mut self, chop: Option<Chop>) -> &mut Self {
        match chop {
            Some(chop) => self
                .clear_bits(chop.cpu_win << CPU_WIN | chop.dma_win << DMA_WIN)
                .set_bits(1 << CHOP | chop.cpu_win << CPU_WIN | chop.dma_win << DMA_WIN),
            None => self.clear_bits(1 << CHOP),
        }
    }

    fn set_mode(&mut self, mode: TransferMode) -> &mut Self {
        self.clear_bits(0b11 << TRANSFER_MODE)
            .set_bits((mode as u32) << TRANSFER_MODE)
    }

    fn start(&mut self) -> &mut Self {
        if let Some(TransferMode::Immediate) = self.get_mode() {
            self.set_bits(1 << TRIGGER);
        }
        self.set_bits(1 << BUSY)
    }

    fn stop(&mut self) -> &mut Self {
        self.clear_bits(1 << BUSY)
    }
}
