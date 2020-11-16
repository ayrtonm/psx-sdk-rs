use crate::dma::{DmaBlocks, GpuDmaAddr, GpuDmaBlock, GpuDmaControl};

impl GpuDmaAddr {
    pub fn read_address(&mut self) -> u32 {
        self.read()
    }

    pub fn set_address(&mut self, mut address: u32) {
        if cfg!(debug_assertions) {
            address &= 0x00FF_FFFF;
        }
        self.write(address);
    }
}

impl GpuDmaBlock {
    // Note that this depends on sync mode, meaning that the channel may not
    // necessarily be in the given block mode
    pub fn set_blocks(&mut self, dma_blocks: DmaBlocks) {
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
