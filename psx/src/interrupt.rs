//! Memory-mapped interrupt status and mask registers.

use crate::mmio::interrupt::{Mask, Stat};
use crate::mmio::register::{Read, Update, Write};
use core::iter;

#[derive(Clone, Copy)]
pub enum IRQ {
    Vblank = 0,
    GPU,
    CDROM,
    DMA,
    Timer0,
    Timer1,
    Timer2,
    Controller,
    SIO,
    SPU,
    Controller2,
}

const ALL_IRQS: [IRQ; 11] = [
    IRQ::Vblank,
    IRQ::GPU,
    IRQ::CDROM,
    IRQ::DMA,
    IRQ::Timer0,
    IRQ::Timer1,
    IRQ::Timer2,
    IRQ::Controller,
    IRQ::SIO,
    IRQ::SPU,
    IRQ::Controller2,
];

impl IntoIterator for IRQ {
    type Item = IRQ;

    type IntoIter = impl Iterator<Item = IRQ>;

    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

impl Mask {
    /// Returns the [`IRQ`]\(s\) enabled by [I_MASK](http://problemkaputt.de/psx-spx.htm#interrupts).
    pub fn enabled(&self) -> impl Iterator<Item = IRQ> {
        let val = unsafe { self.read() };
        ALL_IRQS
            .iter()
            .filter_map(move |&irq| (val & (1 << irq as u32) != 0).then_some(irq))
    }

    /// Returns the [`IRQ`]\(s\) disabled by
    /// [I_MASK](http://problemkaputt.de/psx-spx.htm#interrupts).
    pub fn disabled(&self) -> impl Iterator<Item = IRQ> {
        let val = unsafe { self.read() };
        ALL_IRQS
            .iter()
            .filter_map(move |&irq| (val & (1 << irq as u32) == 0).then_some(irq))
    }

    /// Enables the given [`IRQ`]\(s\) by setting the corresponding bits of
    /// [I_MASK](http://problemkaputt.de/psx-spx.htm#interrupts).
    pub fn enable<I>(&mut self, interrupts: I)
    where I: IntoIterator<Item = IRQ> {
        unsafe {
            self.update(|val| {
                interrupts
                    .into_iter()
                    .fold(val, |val, irq| val | (1 << irq as u32))
            })
        }
    }

    /// Enables all [`IRQ`]s by setting the relevant bits of
    /// [I_MASK](http://problemkaputt.de/psx-spx.htm#interrupts).
    pub fn enable_all(&mut self) {
        unsafe { self.write(0x0000_07FF) }
    }

    /// Disables the given [`IRQ`]\(s\) by clearing the corresponding bits of
    /// [I_MASK](http://problemkaputt.de/psx-spx.htm#interrupts).
    pub fn disable<I>(&mut self, interrupts: I)
    where I: IntoIterator<Item = IRQ> {
        unsafe {
            self.update(|val| {
                interrupts
                    .into_iter()
                    .fold(val, |val, irq| val & !(1 << irq as u32))
            })
        }
    }

    /// Disables all [`IRQ`]s by clearing the relevant bits of
    /// [I_MASK](http://problemkaputt.de/psx-spx.htm#interrupts).
    pub fn disable_all(&mut self) {
        unsafe { self.write(0x0000_0000) }
    }
}

impl Stat {
    /// Zeroes the bit(s) of [I_STAT](http://problemkaputt.de/psx-spx.htm#interrupts) correponding
    /// to the given [`IRQ`]\(s\) to acknowledge them.
    pub fn ack<I>(&mut self, interrupts: I)
    where I: IntoIterator<Item = IRQ> {
        unsafe {
            self.update(|val| {
                interrupts
                    .into_iter()
                    .fold(val, |val, irq| val & !(1 << irq as u32))
            })
        }
    }

    /// Waits until the bit(s) of [I_STAT](http://problemkaputt.de/psx-spx.htm#interrupts)
    /// corresponding to the given [`IRQ`]\(s\) are set.
    pub fn wait<I>(&mut self, interrupts: I)
    where I: IntoIterator<Item = IRQ> {
        let mask = interrupts
            .into_iter()
            .fold(0, |acc, irq| acc | (1 << irq as u32));
        unsafe { while self.read() != mask {} }
    }
}
