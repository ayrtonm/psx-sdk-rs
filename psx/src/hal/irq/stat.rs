use super::ty::IRQ;
use crate::hal::{MutRegister, Mutable, Register, State, I_STAT};

impl<S: State> I_STAT<S> {
    pub fn wait(&mut self, irq: IRQ) {
        while self.all_cleared(1 << (irq as u32)) {
            self.reload();
        }
    }
}

impl I_STAT<Mutable> {
    pub fn ack(&mut self, irq: IRQ) -> &mut Self {
        self.clear(1 << (irq as u32))
    }

    pub fn ack_all(&mut self) -> &mut Self {
        self.clear_all()
    }
}
