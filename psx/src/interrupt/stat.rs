use super::IRQ;
use crate::mmio::interrupt::Stat;
use crate::mmio::register::{Read, Update};

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
