use crate::hw::cop0::{Cause, IntMask};
use crate::hw::Register;
use core::fmt;
use core::fmt::{Debug, Formatter};

impl Cause {
    /// Set an interrupt as pending in the coprocessor 0. This is only useful
    /// for the software interrupts.
    pub fn pending(&self, int_mask: IntMask) -> bool {
        self.all_set(1 << (int_mask as u32))
    }

    /// Acknowledges a pending interrupt in the coprocessor 0.
    pub fn ack(&mut self, int_mask: IntMask) -> &mut Self {
        self.clear_bits(1 << (int_mask as u32))
    }
}

impl Debug for Cause {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("cop0::Cause")
            .field("bits", &self.to_bits())
            .field("hw_interrupt_pending", &self.pending(IntMask::Hardware))
            .field("sw0_interrupt_pending", &self.pending(IntMask::Software0))
            .field("sw1_interrupt_pending", &self.pending(IntMask::Software1))
            .finish()
    }
}
