use crate::registers::{BitTwiddle, RegisterRead, RegisterWrite};
use crate::rw_register;

rw_register!(GpuDmaAddr, 0x1F80_10A0);
rw_register!(GpuDmaBlock, 0x1F80_10A4);
rw_register!(GpuDmaControl, 0x1F80_10A8);

pub enum Blocks {
    // TODO: this should be u32 since 0x10000 is valid and gets mapped to 0u16
    Words(u32),
    Blocks { words: u32, blocks: u32 },
    LinkedList,
}

pub enum Direction {
    ToRam,
    FromRam,
}

pub enum Step {
    Forward,
    Backward,
}

pub trait DmaAddr: RegisterRead + RegisterWrite {
    fn read_address(&mut self) -> u32 {
        self.read()
    }

    fn set_address(&mut self, mut address: u32) {
        if cfg!(debug_assertions) {
            address &= 0x00FF_FFFF;
        }
        self.write(address);
    }
}

pub trait DmaBlock: RegisterRead + RegisterWrite {
    // Note that this depends on sync mode, meaning that the channel may not
    // necessarily be in the given block mode
    fn set_blocks(&mut self, dma_blocks: Blocks) {
        match dma_blocks {
            Blocks::Words(words) => {
                // TODO: I should be doing the opposite here
                let words = if words == 0 {
                    0x0001_0000
                } else {
                    words.into()
                };
                self.write(words);
            },
            Blocks::Blocks { words, blocks } => {
                self.write(words as u32 | ((blocks as u32) << 16))
            },
            Blocks::LinkedList => self.write(0),
        }
    }
}

pub trait DmaControl: RegisterRead + RegisterWrite {
    fn set_direction(&mut self, direction: Direction) {
        let bit = match direction {
            Direction::ToRam => 0,
            Direction::FromRam => 1,
        };
        let current_value = self.read();
        let new_value = current_value.clear(0) | bit;
        self.write(new_value);
    }

    fn set_step(&mut self, step: Step) {
        let bit = match step {
            Step::Forward => 0,
            Step::Backward => 1,
        };
        let current_value = self.read();
        let new_value = current_value.clear(1) | (bit << 1);
        self.write(new_value);
    }
    fn enable_chopping(&mut self) {
    }
}

impl DmaAddr for GpuDmaAddr {}
impl DmaBlock for GpuDmaBlock {}
impl DmaControl for GpuDmaControl {}
