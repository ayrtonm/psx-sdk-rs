//! Interrupt request and acknowledge
use crate::hw::{MemRegister, Register};
use crate::irq::IRQ;
use core::mem::variant_count;

/// Interrupt status register
pub type Status = MemRegister<u16, 0x1F80_1070>;
/// Interrupt mask register
pub type Mask = MemRegister<u16, 0x1F80_1074>;

const ALL_IRQS: u16 = (1 << variant_count::<IRQ>()) - 1;

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
        self.clear_bits(ALL_IRQS)
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
        self.set_bits(ALL_IRQS)
    }

    /// Disables all interrupts in the interrupt request mask register.
    pub fn disable_all(&mut self) -> &mut Self {
        self.clear_bits(ALL_IRQS)
    }
}
