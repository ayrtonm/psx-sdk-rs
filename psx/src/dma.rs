use crate::mmio::{Load, Store};
use crate::value::{MutValue, Value};

declare_rw!(
    /// [DMA Control](http://problemkaputt.de/psx-spx.htm#dmachannels) register at `0x1F80_10F0`.
    /// Used to enable DMA channels and set priorities.
    Control,
    0x1F80_10F0,
    u32
);

declare_rw!(
    /// [DMA Interrupt](http://problemkaputt.de/psx-spx.htm#dmachannels) register at `0x1F80_10F4`.
    /// Used to enable and force DMA IRQs.
    Interrupt,
    0x1F80_10F4,
    u32
);

/// A [DMA channel](http://problemkaputt.de/psx-spx.htm#dmachannels).
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    /// DMA channel for RAM to Macroblock Decoder transfers.
    MDECin = 0,
    /// DMA channel for Macroblock Decoder to RAM transfers.
    MDECout,
    /// DMA channel for Graphics Processing Unit linked lists and image data.
    GPU,
    /// DMA channel for CDROM to RAM transfers.
    CDROM,
    /// DMA channel for RAM to Sound Processing Unit transfers.
    SPU,
    /// DMA channel for Expansion port transfers.
    PIO,
    /// DMA channel for clearing GPU ordering tables.
    OTC,
}

impl MutValue<'_, Control, u32> {
    /// Enables the given DMA channel.
    #[inline(always)]
    pub fn enable(mut self, ch: Channel) -> Self {
        let bit = (ch as u32 * 4) + 3;
        self.bits |= 1 << bit;
        self
    }

    /// Disables the given DMA channel.
    #[inline(always)]
    pub fn disable(mut self, ch: Channel) -> Self {
        let bit = (ch as u32 * 4) + 3;
        self.bits &= !(1 << bit);
        self
    }
}

/// Marker for DMA channel base address registers.
pub trait BaseAddress: Load<u32> + Store<u32> {}

impl<R: BaseAddress> Value<R, u32> {
    /// Gets the DMA channel's current base address.
    #[inline(always)]
    pub fn get_address(&self) -> u32 {
        self.bits
    }
}

impl<R: BaseAddress> MutValue<'_, R, u32> {
    /// Sets the DMA channel's base address.
    pub fn set_address(mut self, addr: u32) -> Self {
        self.bits = addr;
        self
    }
}

/// Methods for using the Graphics Processing Unit DMA channel.
pub mod gpu {
    use super::BaseAddress as BaseAddressTrait;

    declare_rw!(
        /// [GPU DMA base address](http://problemkaputt.de/psx-spx.htm#dmachannels) register at `0x1F80_10A0`.
        /// Used to set the DMA channel's base address.
        BaseAddress,
        0x1F80_10A0,
        u32
    );

    impl BaseAddressTrait for BaseAddress {}
}
