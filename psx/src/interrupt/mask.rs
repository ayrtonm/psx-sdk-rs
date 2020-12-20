use super::{ALL_IRQS, IRQ};
use crate::mmio::irq;
use crate::mmio::register::{Read, Write};

impl_mut_value!(irq::Mask);

impl Value {
    /// Returns the [`IRQ`]\(s\) enabled by [I_MASK](http://problemkaputt.de/psx-spx.htm#interrupts).
    #[inline(always)]
    pub fn enabled(&self) -> impl Iterator<Item = IRQ> + '_ {
        ALL_IRQS
            .iter()
            .filter_map(move |&irq| (self.bits & (1 << irq as u32) != 0).then_some(irq))
    }

    /// Returns the [`IRQ`]\(s\) disabled by
    /// [I_MASK](http://problemkaputt.de/psx-spx.htm#interrupts).
    #[inline(always)]
    pub fn disabled(&self) -> impl Iterator<Item = IRQ> + '_ {
        ALL_IRQS
            .iter()
            .filter_map(move |&irq| (self.bits & (1 << irq as u32) == 0).then_some(irq))
    }
}

impl<'a> MutValue<'a> {
    /// Enables the given [`IRQ`]\(s\) by setting the corresponding bits of
    /// [I_MASK](http://problemkaputt.de/psx-spx.htm#interrupts).
    #[inline(always)]
    pub fn enable<I>(mut self, interrupts: I) -> Self
    where I: IntoIterator<Item = IRQ> {
        self.value.bits = interrupts
            .into_iter()
            .fold(self.value.bits, |val, irq| val | (1 << irq as u32));
        self
    }

    /// Enables all [`IRQ`]s by setting the relevant bits of
    /// [I_MASK](http://problemkaputt.de/psx-spx.htm#interrupts).
    #[inline(always)]
    pub fn enable_all(mut self) -> Self {
        self.value.bits = 0x0000_07FF;
        self
    }

    /// Disables the given [`IRQ`]\(s\) by clearing the corresponding bits of
    /// [I_MASK](http://problemkaputt.de/psx-spx.htm#interrupts).
    #[inline(always)]
    pub fn disable<I>(mut self, interrupts: I) -> Self
    where I: IntoIterator<Item = IRQ> {
        self.value.bits = interrupts
            .into_iter()
            .fold(self.value.bits, |val, irq| val & !(1 << irq as u32));
        self
    }

    /// Disables all [`IRQ`]s by clearing the relevant bits of
    /// [I_MASK](http://problemkaputt.de/psx-spx.htm#interrupts).
    #[inline(always)]
    pub fn disable_all(mut self) -> Self {
        self.value.bits = 0;
        self
    }
}
