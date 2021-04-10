use super::ty::IRQ;
use super::ALL_IRQS;
use crate::hal::{MutRegister, Mutable, Register, State, I_MASK};

impl<S: State> I_MASK<S> {
    pub fn irq_enabled(&self, irq: IRQ) -> bool {
        self.all_set(1 << (irq as u32))
    }

    pub fn irq_disabled(&self, irq: IRQ) -> bool {
        self.all_cleared(1 << (irq as u32))
    }
}

impl I_MASK<Mutable> {
    pub fn enable_irq(&mut self, irq: IRQ) -> &mut Self {
        self.set_bits(1 << (irq as u32))
    }

    pub fn disable_irq(&mut self, irq: IRQ) -> &mut Self {
        self.clear_bits(1 << (irq as u32))
    }

    pub fn enable_all(&mut self) -> &mut Self {
        self.set_bits(ALL_IRQS)
    }

    pub fn disable_all(&mut self) -> &mut Self {
        self.clear_bits(ALL_IRQS)
    }
}
