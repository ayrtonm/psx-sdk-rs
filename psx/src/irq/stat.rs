use super::IRQ;

use crate::mmio::Address;
use crate::value;
use crate::value::{Load, LoadMut};

/// [Interrupt stat](http://problemkaputt.de/psx-spx.htm#interrupts) register at `0x1F80_1070`.
/// Used to acknowledge and wait for IRQs.
pub struct ISTAT;

impl Address<u32> for ISTAT {
    const ADDRESS: u32 = 0x1F80_1070;
}
impl LoadMut<u32> for ISTAT {}

/// A [`value::Value`] alias for the interrupt request status register.
type Value<'r> = value::Value<'r, u32, ISTAT>;
/// A [`value::MutValue`] alias for the interrupt request status register.
type MutValue<'r> = value::MutValue<'r, u32, ISTAT>;

impl ISTAT {
    /// Wait for the given interrupt request.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn wait(&self, irq: IRQ) {
        while !self.load().requested(irq) {}
    }
}

impl Value<'_> {
    /// Checks if the given interrupt has been requested.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn requested(&self, irq: IRQ) -> bool {
        self.contains(1 << (irq as u32))
    }
}

impl MutValue<'_> {
    /// Acknowledge the given interrupt reqest.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn ack(self, irq: IRQ) -> Self {
        self.clear(1 << (irq as u32))
    }

    /// Acknowledge all interrupt requests.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn ack_all(self) -> Self {
        self.clear_all()
    }
}
