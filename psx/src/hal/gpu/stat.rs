use crate::hal::{Register, GPUSTAT};

const CMD_READY: u32 = 26;
const DMA_READY: u32 = 28;

impl GPUSTAT {
    pub fn cmd_ready(&self) -> bool {
        self.contains(1 << CMD_READY)
    }

    pub fn dma_ready(&self) -> bool {
        self.contains(1 << DMA_READY)
    }

    pub fn wait_cmd(&mut self) -> &mut Self {
        while !self.cmd_ready() {
            self.reload();
        }
        self
    }

    pub fn wait_dma(&mut self) -> &mut Self {
        while !self.dma_ready() {
            self.reload();
        }
        self
    }
}
