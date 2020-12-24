use crate::mmio::Address;
use crate::value;
use crate::value::Load;

/// [GPUStat](http://problemkaputt.de/psx-spx.htm#gpuioportsdmachannelscommandsvram) register at `0x1F80_1814`.
/// Used to check GPU status.
pub struct GPUStat;

impl Address<u32> for GPUStat {
    const ADDRESS: u32 = 0x1F80_1814;
}

impl Load<u32> for GPUStat {}

impl GPUStat {
    const CMD_READY: u32 = 26;
    const DMA_READY: u32 = 28;
    const DMA_DIRECTION: u32 = 29;
}

/// A [`value::Value`] alias for the GPU status register.
pub type Value<'r> = value::Value<'r, u32, GPUStat>;

impl Value<'_> {
    /// Checks if DMA is enabled for the GPU.
    #[inline(always)]
    pub fn dma_enabled(&self) -> bool {
        self.any(0b11 << GPUStat::DMA_DIRECTION)
    }

    /// Checks if the GPU is ready to receive a DMA block.
    #[inline(always)]
    pub fn dma_ready(&self) -> bool {
        self.contains(1 << GPUStat::DMA_READY)
    }

    /// Checks if the GPU is ready to receive a command.
    #[inline(always)]
    pub fn cmd_ready(&self) -> bool {
        self.contains(1 << GPUStat::CMD_READY)
    }
}
