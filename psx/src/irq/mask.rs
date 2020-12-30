use super::IRQ;

use crate::mmio::Address;
use crate::value;
use crate::value::LoadMut;

/// [Interrupt request mask](http://problemkaputt.de/psx-spx.htm#interrupts) register at `0x1F80_1074`.
/// Used to enable and disable IRQs.
pub struct IMASK;

impl Address<u32> for IMASK {
    const ADDRESS: u32 = 0x1F80_1074;
}
impl LoadMut<u32> for IMASK {}

/// A [`value::Value`] alias for the interrupt request mask register.
type Value<'r> = value::Value<'r, u32, IMASK>;
/// A [`value::MutValue`] alias for the interrupt request mask register.
type MutValue<'r> = value::MutValue<'r, u32, IMASK>;

impl Value<'_> {
    /// Checks if the given IRQ is enabled.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn enabled(&self, irq: IRQ) -> bool {
        self.contains(1 << (irq as u32))
    }

    /// Checks if the given IRQ is disabled.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn disabled(&self, irq: IRQ) -> bool {
        !self.contains(1 << (irq as u32))
    }
}

impl MutValue<'_> {
    /// Enables the given IRQ.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn enable(self, irq: IRQ) -> Self {
        self.set(1 << (irq as u32))
    }

    /// Disable the given IRQ.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn disable(self, irq: IRQ) -> Self {
        self.clear(1 << (irq as u32))
    }

    /// Enables all IRQs.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn enable_all(mut self) -> Self {
        self.value.bits = 0x0000_07FF;
        self
    }

    /// Disables all IRQs.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn disable_all(self) -> Self {
        self.clear_all()
    }
}
