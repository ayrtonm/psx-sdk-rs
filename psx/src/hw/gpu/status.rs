use crate::gpu::VideoMode;
use crate::hw::gpu::Status;
use crate::hw::{MemRegister, Register};
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

impl Status {
    /// Creates a new handle and immediately reads the register's value.
    ///
    /// This does a single volatile read.
    pub fn new() -> Self {
        Status(MemRegister::new())
    }

    /// Load the register's value into a cache.
    ///
    /// This does a single volatile read.
    pub fn load(&mut self) -> &mut Self {
        self.0.load();
        self
    }

    /// Checks the video mode bit.
    pub fn video_mode(&self) -> VideoMode {
        if self.0.all_set(1 << VIDEO_MODE) {
            VideoMode::PAL
        } else {
            VideoMode::NTSC
        }
    }

    /// Checks the interlaced bit.
    pub fn interlaced(&self) -> bool {
        self.0.all_set(1 << INTERLACE)
    }

    /// Checks if the display is enabled.
    pub fn display_enabled(&self) -> bool {
        !self.0.all_set(1 << DISPLAY_ENABLE)
    }

    /// Checks for a pending GPU interrupt request.
    pub fn irq_pending(&self) -> bool {
        self.0.all_set(1 << IRQ)
    }

    /// Checks the command ready bit.
    pub fn cmd_ready(&self) -> bool {
        self.0.all_set(1 << CMD_READY)
    }

    /// Checks the DMA ready bit.
    pub fn dma_ready(&self) -> bool {
        self.0.all_set(1 << DMA_READY)
    }

    /// Checks if DMA is enabled.
    pub fn dma_enabled(&self) -> bool {
        self.0.any_set(0b11 << DMA_DIRECTION)
    }

    /// Checks if an odd-numbered line is being drawn.
    pub fn odd_line(&self) -> bool {
        self.0.all_set(1 << LINE_PARITY)
    }

    /// Checks if an even-numbered line is being drawn.
    pub fn even_line(&self) -> bool {
        !self.odd_line()
    }

    /// Waits until the GPU is ready to receive a command. This loops and
    /// reloads the GPUSTAT register until it's done waiting.
    pub fn wait_cmd(&mut self) -> &mut Self {
        while !self.cmd_ready() {
            self.0.load();
        }
        self
    }

    /// Waits until the GPU DMA is ready. This loops and reloads the GPUSTAT
    /// register until it's done waiting.
    pub fn wait_dma(&mut self) -> &mut Self {
        while !self.dma_ready() {
            self.0.load();
        }
        self
    }

    #[cfg(test)]
    pub(crate) fn averaged_bits(&self) -> u32 {
        self.0.to_bits() & !(1 << LINE_PARITY)
    }
}

impl Debug for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("GPUSTAT")
            .field("bits", &self.0.to_bits())
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
