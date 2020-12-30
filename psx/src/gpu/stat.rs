use super::{Parity, VideoMode};

use crate::mmio::Address;
use crate::value;
use crate::value::Load;

/// [GPUSTAT](http://problemkaputt.de/psx-spx.htm#gpuioportsdmachannelscommandsvram) register at `0x1F80_1814`.
/// Used to check GPU status.
pub struct GPUSTAT;

impl Address<u32> for GPUSTAT {
    const ADDRESS: u32 = 0x1F80_1814;
}

impl Load<u32> for GPUSTAT {}

impl GPUSTAT {
    const VIDEO_MODE: u32 = 20;
    const INTERLACE: u32 = 22;
    const CMD_READY: u32 = 26;
    const DMA_READY: u32 = 28;
    const DMA_DIRECTION: u32 = 29;
    const LINE_PARITY: u32 = 31;
}

/// A [`value::Value`] alias for the GPU status register.
pub type Value<'r> = value::Value<'r, u32, GPUSTAT>;

impl Value<'_> {
    /// Checks if video is interlaced.
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    pub fn interlaced(&self) -> bool {
        self.contains(1 << GPUSTAT::INTERLACE)
    }

    /// Checks if DMA is enabled for the GPU.
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    pub fn dma_enabled(&self) -> bool {
        self.any(0b11 << GPUSTAT::DMA_DIRECTION)
    }

    /// Checks if the GPU is ready to receive a DMA block.
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    pub fn dma_ready(&self) -> bool {
        self.contains(1 << GPUSTAT::DMA_READY)
    }

    /// Checks if the GPU is ready to receive a command.
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    pub fn cmd_ready(&self) -> bool {
        self.contains(1 << GPUSTAT::CMD_READY)
    }

    /// Gets the current video mode.
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    pub fn video_mode(&self) -> VideoMode {
        if self.contains(1 << GPUSTAT::VIDEO_MODE) {
            VideoMode::PAL
        } else {
            VideoMode::NTSC
        }
    }

    /// Gets the parity of the line being drawn.
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    pub fn line(&self) -> Parity {
        if self.contains(1 << GPUSTAT::LINE_PARITY) {
            Parity::Odd
        } else {
            Parity::Even
        }
    }
}
