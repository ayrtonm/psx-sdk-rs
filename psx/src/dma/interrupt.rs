use crate::mmio::Address;
use crate::value;
use crate::value::LoadMut;

/// [DMA Interrupt](http://problemkaputt.de/psx-spx.htm#dmachannels) register at `0x1F80_10F4`.
/// Used to enable and force DMA IRQs.
pub struct Interrupt;
/// A [`value::Value`] alias for [`Interrupt`].
pub type Value<'r> = value::Value<'r, u32, Interrupt>;
/// A [`value::MutValue`] alias for [`Interrupt`].
pub type MutValue<'r> = value::MutValue<'r, u32, Interrupt>;

impl Address<u32> for Interrupt {
    const ADDRESS: u32 = 0x1F80_10F4;
}

impl LoadMut<u32> for Interrupt {}
