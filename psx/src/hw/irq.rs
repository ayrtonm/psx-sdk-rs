//! Interrupt request and acknowledge
use crate::hw::{MemRegister, Register};
use crate::irq::{ALL_IRQS, IRQ, NUM_IRQS};

/// Interrupt status register
pub type Status = MemRegister<u16, 0x1F80_1070>;
/// Interrupt mask register
pub type Mask = MemRegister<u16, 0x1F80_1074>;

/// A bitmask for the currently requested interrupts
#[derive(Clone, Copy, Debug)]
pub struct Requested(u16);

impl Requested {
    /// Create a new interrupt requested bitmask
    pub const fn new(value: u16) -> Self {
        Self(value)
    }
    /// Set the interrupt requested bit for the given IRQ
    pub fn set(&mut self, irq: IRQ) {
        self.0 |= 1 << (irq as u16);
    }
    /// Clear the interrupt requested bit for the given IRQ
    pub fn clear(&mut self, irq: IRQ) {
        self.0 &= !(1 << (irq as u16));
    }

    /// Gets an iterator over the requested interrupt bits
    pub fn iter(&self) -> impl Iterator<Item = IRQ> {
        IRQIter {
            value: *self,
            iter_idx: 0,
        }
    }
}

/// An iterator over requested interrupt bits in a bitmask
pub struct IRQIter {
    value: Requested,
    iter_idx: usize,
}

impl Iterator for IRQIter {
    type Item = IRQ;
    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_idx == NUM_IRQS {
            return None
        }
        let this_bit = self.iter_idx;
        let bit_value = self.value.0 & (1 << this_bit);
        self.iter_idx += 1;
        if bit_value != 0 {
            Some(ALL_IRQS[this_bit])
        } else {
            None
        }
    }
}

const ALL_IRQS_BITS: u16 = (1 << NUM_IRQS) - 1;

impl Status {
    /// Requests an interrupt in the interrupt status register.
    pub fn requested(&self, irq: IRQ) -> bool {
        self.all_set(1 << (irq as u32))
    }

    /// Acknowledges an interrupt in the interrupt status register.
    pub fn ack(&mut self, irq: IRQ) -> &mut Self {
        self.clear_bits(1 << (irq as u32))
    }

    /// Acknowledges all interrupts in the interrupt status register.
    pub fn ack_all(&mut self) -> &mut Self {
        self.clear_bits(ALL_IRQS_BITS)
    }

    /// Waits for an interrupt to be requested in the interrupt status register.
    pub fn wait(&mut self, irq: IRQ) -> &mut Self {
        while !self.requested(irq) {
            self.load();
        }
        self
    }
}

impl Mask {
    /// Checks if an interrupt is enabled in the interrupt request mask
    /// register.
    pub fn irq_enabled(&self, irq: IRQ) -> bool {
        self.all_set(1 << (irq as u32))
    }

    /// Checks if an interrupt is disabled in the interrupt request mask
    /// register.
    pub fn irq_disabled(&self, irq: IRQ) -> bool {
        !self.irq_enabled(irq)
    }

    /// Enables an interrupt in the interrupt request mask register.
    pub fn enable_irq(&mut self, irq: IRQ) -> &mut Self {
        self.set_bits(1 << (irq as u32))
    }

    /// Disables an interrupt in the interrupt request mask register.
    pub fn disable_irq(&mut self, irq: IRQ) -> &mut Self {
        self.clear_bits(1 << (irq as u32))
    }

    /// Enables all interrupts in the interrupt request mask register.
    pub fn enable_all(&mut self) -> &mut Self {
        self.set_bits(ALL_IRQS_BITS)
    }

    /// Disables all interrupts in the interrupt request mask register.
    pub fn disable_all(&mut self) -> &mut Self {
        self.clear_bits(ALL_IRQS_BITS)
    }

    /// Returns an array of IRQs that are both enabled and requested.
    ///
    /// This returns an array to avoid dynamic allocation. Elements that are
    /// `None` may be ignored.
    pub fn active_irqs(&self, stat: &Status) -> Requested {
        Requested(self.to_bits() & stat.to_bits())
    }
}
