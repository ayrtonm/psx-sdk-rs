use crate::gpu::VideoMode;
use crate::hal::{Register, GPUSTAT};

const VIDEO_MODE: u32 = 20;
const INTERLACE: u32 = 22;
const CMD_READY: u32 = 26;
const DMA_READY: u32 = 28;
const DMA_DIRECTION: u32 = 29;

impl GPUSTAT {
    pub fn video_mode(&self) -> VideoMode {
        if self.contains(1 << VIDEO_MODE) {
            VideoMode::PAL
        } else {
            VideoMode::NTSC
        }
    }

    pub fn interlaced(&self) -> bool {
        self.contains(1 << INTERLACE)
    }

    pub fn cmd_ready(&self) -> bool {
        self.contains(1 << CMD_READY)
    }

    pub fn dma_ready(&self) -> bool {
        self.contains(1 << DMA_READY)
    }

    pub fn dma_enabled(&self) -> bool {
        self.any(0b11 << DMA_DIRECTION)
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

#[cfg(test)]
mod tests {
    #[test_case]
    fn bios_fn_() {
        use crate::bios;
        use crate::hal::GPUSTAT;
        assert!(bios::gpu_get_status() == GPUSTAT::load());
    }
}
