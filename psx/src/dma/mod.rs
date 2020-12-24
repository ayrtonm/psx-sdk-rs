use crate::value;
use crate::value::LoadMut;

/// DMA control register. Used to enable DMA channels and set priorities.
pub mod control;
/// DMA interrupt register. Used to enable and force DMA IRQs.
pub mod interrupt;
/// Methods for using the Graphics Processing Unit DMA channel.
pub mod gpu;

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

/// A marker for DMA base address registers.
pub trait BaseAddress: LoadMut<u32> {}

/// A marker for DMA block control registers.
pub trait BlockControl: LoadMut<u32> {}

/// A marker for DMA channel control registers.
pub trait ChannelControl: LoadMut<u32> {}

/// A [`value::Value`] alias for DMA channel registers.
pub type Value<'r, R> = value::Value<'r, u32, R>;
/// A [`value::MutValue`] alias for DMA channel registers.
pub type MutValue<'r, R> = value::MutValue<'r, u32, R>;

impl<R: BaseAddress> Value<'_, R> {
    /// Gets the DMA channel's base address.
    #[inline(always)]
    pub fn get_address(&self) -> u32 {
        self.bits
    }
}

impl<R: BaseAddress> MutValue<'_, R> {
    /// Sets the DMA channel's base address.
    #[inline(always)]
    pub fn set_address(mut self, addr: u32) -> Self {
        self.value.bits = addr;
        self
    }
}

impl<R: BlockControl> Value<'_, R> {
}

impl<R: ChannelControl> Value<'_, R> {
}
