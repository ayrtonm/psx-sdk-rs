use crate::registers::{Read, Write, Update};
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

pub trait DmaAddr: Read + Write {
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

pub trait DmaBlock: Read + Write {
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

pub trait DmaControl: Update {
    fn set_direction(&mut self, direction: Direction) {
        let bit = match direction {
            Direction::ToRam => 0,
            Direction::FromRam => 1,
        };
        self.update(0, bit);
    }

    fn set_step(&mut self, step: Step) {
        let bit = match step {
            Step::Forward => 0,
            Step::Backward => 1,
        };
        self.update(1, bit);
    }
    fn set_chopping(&mut self, chop: bool) {
        if chop {
            self.enable_chopping();
        } else {
            self.disable_chopping();
        }
    }
    fn enable_chopping(&mut self) {
        self.update(8, 1);
    }
    fn disable_chopping(&mut self) {
        self.update(8, 0);
    }
}

impl DmaAddr for GpuDmaAddr {}
impl DmaBlock for GpuDmaBlock {}
impl DmaControl for GpuDmaControl {}
