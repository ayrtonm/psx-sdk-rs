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

pub enum Mode {
    Immediate,
    Request,
    LinkedList,
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
        //TODO: add debug mode checks
        match dma_blocks {
            Blocks::Words(words) => {
                let words = if words == 0x1_0000 {
                    0
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
        let bit = if chop {
            1
        } else {
            0
        };
        self.update(8, bit);
    }
    fn set_sync_mode(&mut self, mode: Mode) {
        let bits = match mode {
            Mode::Immediate => 0,
            Mode::Request => 1,
            Mode::LinkedList => 2,
        };
        self.update_bits(9..=10, bits);
    }
}

impl DmaAddr for GpuDmaAddr {}
impl DmaBlock for GpuDmaBlock {}
impl DmaControl for GpuDmaControl {}
