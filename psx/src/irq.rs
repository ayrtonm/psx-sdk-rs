use crate::mmio::Address;
use crate::value::LoadMut;

/// [Interrupt mask](http://problemkaputt.de/psx-spx.htm#interrupts) register at `0x1F80_1074`.
/// Used to enable and disable IRQs.
pub struct IMASK;

impl Address<u32> for IMASK {
    const ADDRESS: u32 = 0x1F80_1074;
}
impl LoadMut<u32> for IMASK {}
