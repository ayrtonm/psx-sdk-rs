use crate::hw::cop0::{Cause, Excode, IntSrc};
use crate::hw::Register;
use core::fmt;
use core::fmt::{Debug, Formatter};

const EXCODE: usize = 2;
const BD: usize = 31;

impl Cause {
    /// Set an interrupt as pending in the coprocessor 0. This is only useful
    /// for the software interrupts.
    pub fn pending(&self, int_src: IntSrc) -> bool {
        self.all_set(1 << (int_src as u32))
    }

    /// Acknowledges a pending interrupt in the coprocessor 0.
    pub fn ack(&mut self, int_src: IntSrc) -> &mut Self {
        self.clear_bits(1 << (int_src as u32))
    }

    /// Checks if the last interrupt occurred in a branch delay slot
    pub fn branch_delay_slot(&self) -> bool {
        self.all_set(1 << BD)
    }

    /// Checks the exception cause code
    pub fn excode(&self) -> Excode {
        match (self.to_bits() >> EXCODE) & 0x1F {
            0x00 => Excode::Interrupt,
            0x04 => Excode::AddressErrorLoad,
            0x05 => Excode::AddressErrorStore,
            0x08 => Excode::Syscall,
            0x0A => Excode::ReservedInstruction,
            _ => Excode::Other,
        }
    }
}

impl Debug for Cause {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("cop0::Cause")
            .field("bits", &self.to_bits())
            .field("hw_interrupt_pending", &self.pending(IntSrc::Hardware))
            .field("sw0_interrupt_pending", &self.pending(IntSrc::Software0))
            .field("sw1_interrupt_pending", &self.pending(IntSrc::Software1))
            .field("branch_delay_slot", &self.branch_delay_slot())
            .field("exception_code", &self.excode())
            .finish()
    }
}
