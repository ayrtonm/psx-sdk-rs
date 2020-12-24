use super::Channel;

use crate::mmio::Address;
use crate::value;
use crate::value::LoadMut;

/// [DMA Control](http://problemkaputt.de/psx-spx.htm#dmachannels) register at `0x1F80_10F0`.
/// Used to enable DMA channels and set priorities.
pub struct Control;
/// A [`value::Value`] alias for [`Control`].
pub type Value<'r> = value::Value<'r, u32, Control>;
/// A [`value::MutValue`] alias for [`Control`].
pub type MutValue<'r> = value::MutValue<'r, u32, Control>;

impl Address<u32> for Control {
    const ADDRESS: u32 = 0x1F80_10F0;
}
impl LoadMut<u32> for Control {}

impl Control {
    #[inline(always)]
    fn enable_bit(ch: Channel) -> u32 {
        let bit = (ch as u32 * 4) + 3;
        1 << bit
    }
}

impl Value<'_> {
    /// Checks if the given DMA channel is enabled.
    #[inline(always)]
    pub fn enabled(&self, ch: Channel) -> bool {
        self.contains(Control::enable_bit(ch))
    }
}

impl MutValue<'_> {
    /// Enables the given DMA channel.
    #[inline(always)]
    pub fn enable(self, ch: Channel) -> Self {
        self.set(Control::enable_bit(ch))
    }

    /// Disables the given DMA channel.
    #[inline(always)]
    pub fn disable(self, ch: Channel) -> Self {
        self.clear(Control::enable_bit(ch))
    }
}
