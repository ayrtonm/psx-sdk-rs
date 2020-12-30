use super::Channel;

use crate::mmio::Address;
use crate::value;
use crate::value::LoadMut;

/// [DMA Control](http://problemkaputt.de/psx-spx.htm#dmachannels) register at `0x1F80_10F0`.
/// Used to enable DMA channels and set priorities.
pub struct DPCR;
/// A [`value::Value`] alias for [`DPCR`].
pub type Value<'r> = value::Value<'r, u32, DPCR>;
/// A [`value::MutValue`] alias for [`DPCR`].
pub type MutValue<'r> = value::MutValue<'r, u32, DPCR>;

impl Address<u32> for DPCR {
    const ADDRESS: u32 = 0x1F80_10F0;
}
impl LoadMut<u32> for DPCR {}

#[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
const fn enable_bit(ch: Channel) -> u32 {
    let bit = (ch as u32 * 4) + 3;
    1 << bit
}

const ENABLE_BITS: u32 = {
    enable_bit(Channel::MDECin) |
        enable_bit(Channel::MDECout) |
        enable_bit(Channel::GPU) |
        enable_bit(Channel::CDROM) |
        enable_bit(Channel::SPU) |
        enable_bit(Channel::PIO) |
        enable_bit(Channel::OTC)
};

impl Value<'_> {
    /// Checks if the given DMA channel is enabled.
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    pub fn enabled(&self, ch: Channel) -> bool {
        self.contains(enable_bit(ch))
    }
}

impl MutValue<'_> {
    /// Enables the given DMA channel.
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    pub fn enable(self, ch: Channel) -> Self {
        self.set(enable_bit(ch))
    }

    /// Disables the given DMA channel.
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    pub fn disable(self, ch: Channel) -> Self {
        self.clear(enable_bit(ch))
    }

    /// Enables all DMA channels.
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    pub fn enable_all(self) -> Self {
        self.set(ENABLE_BITS)
    }

    /// Disables all DMA channels.
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    pub fn disable_all(self) -> Self {
        self.clear(ENABLE_BITS)
    }
}
