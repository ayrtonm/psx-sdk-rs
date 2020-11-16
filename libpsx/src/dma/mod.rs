use crate::macros::{RegisterRead, RegisterWrite};
use crate::rw_register;

rw_register!(GpuDmaAddr, 0x1F80_10A0);
rw_register!(GpuDmaBlock, 0x1F80_10A4);
rw_register!(GpuDmaControl, 0x1F80_10A8);

pub enum DmaBlocks {
    // TODO: this should be u32 since 0x10000 is valid and gets mapped to 0u16
    Words(u32),
    Blocks { words: u32, blocks: u32 },
    LinkedList,
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
    fn set_blocks(&mut self, dma_blocks: DmaBlocks) {
        match dma_blocks {
            DmaBlocks::Words(words) => {
                // TODO: I should be doing the opposite here
                let words = if words == 0 {
                    0x0001_0000
                } else {
                    words.into()
                };
                self.write(words);
            },
            DmaBlocks::Blocks { words, blocks } => {
                self.write(words as u32 | ((blocks as u32) << 16))
            },
            DmaBlocks::LinkedList => self.write(0),
        }
    }
}

pub trait DmaControl: RegisterRead + RegisterWrite {}

impl DmaAddr for GpuDmaAddr {}
impl DmaBlock for GpuDmaBlock {}
impl DmaControl for GpuDmaControl {}
