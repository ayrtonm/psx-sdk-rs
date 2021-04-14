use super::ty::VideoMode;
use crate::hal::{Register, GPUSTAT};
use core::fmt;
use core::fmt::{Debug, Formatter};

const VIDEO_MODE: u32 = 20;
const INTERLACE: u32 = 22;
const DISPLAY_ENABLE: u32 = 23;
const IRQ: u32 = 24;
const CMD_READY: u32 = 26;
const DMA_READY: u32 = 28;
const DMA_DIRECTION: u32 = 29;
const LINE_PARITY: u32 = 31;

impl GPUSTAT {
    pub fn video_mode(&self) -> VideoMode {
        if self.all_set(1 << VIDEO_MODE) {
            VideoMode::PAL
        } else {
            VideoMode::NTSC
        }
    }

    pub fn interlaced(&self) -> bool {
        self.all_set(1 << INTERLACE)
    }

    pub fn display_enabled(&self) -> bool {
        !self.all_set(1 << DISPLAY_ENABLE)
    }

    pub fn irq_pending(&self) -> bool {
        self.all_set(1 << IRQ)
    }

    pub fn cmd_ready(&self) -> bool {
        self.all_set(1 << CMD_READY)
    }

    pub fn dma_ready(&self) -> bool {
        self.all_set(1 << DMA_READY)
    }

    pub fn dma_enabled(&self) -> bool {
        self.any_set(0b11 << DMA_DIRECTION)
    }

    pub fn odd_line(&self) -> bool {
        self.all_set(1 << LINE_PARITY)
    }

    pub fn even_line(&self) -> bool {
        !self.odd_line()
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

    pub fn bits_no_parity(&self) -> u32 {
        self.bits() & !(1 << LINE_PARITY)
    }
}

impl Debug for GPUSTAT {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("GPUSTAT")
            .field("bits", &self.bits())
            .field("video_mode", &self.video_mode())
            .field("interlaced", &self.interlaced())
            .field("display_enabled", &self.display_enabled())
            .field("irq_pending", &self.irq_pending())
            .field("cmd_ready", &self.cmd_ready())
            .field("dma_ready", &self.dma_ready())
            .field("dma_enabled", &self.dma_enabled())
            .field("odd_line", &self.odd_line())
            .field("even_line", &self.even_line())
            .finish()
    }
}
