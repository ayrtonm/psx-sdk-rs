//! Interrupt request and acknowledge
use crate::hw::{MemRegister, Register};
use crate::irq::{ALL_IRQS, IRQ, NUM_IRQS};

/// Interrupt status register
pub type Status = MemRegister<u16, 0x1F80_1070>;
/// Interrupt mask register
pub type Mask = MemRegister<u16, 0x1F80_1074>;

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
    pub fn active_irqs(&self, stat: &Status) -> [Option<IRQ>; NUM_IRQS] {
        let active_irqs = self.to_bits() & stat.to_bits();
        let mut res = [None; NUM_IRQS];
        for i in 0..NUM_IRQS {
            if active_irqs & (1 << i) != 0 {
                res[i] = Some(ALL_IRQS[i]);
            }
        }
        res
    }
}
