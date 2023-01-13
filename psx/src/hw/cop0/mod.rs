//! System Control Coprocessor
//!
//! This module provides access to registers and instructions in the system
//! control coprocessor, cop0.

use crate::hw::cop0::status::{IM_HW, IM_SW0, IM_SW1};
use crate::hw::Register;
use core::fmt;
use core::fmt::{Debug, Formatter};

mod cause;
mod epc;
mod status;

/// Privilege mode
#[derive(Debug)]
pub enum Mode {
    /// Kernel mode
    Kernel = 0,
    /// User mode
    User = 1,
}

/// The source of the interrupt that caused an exception.
#[repr(u32)]
pub enum IntSrc {
    /// One software-based interrupt
    Software0 = IM_SW0,
    /// The other software-based interrupt
    Software1 = IM_SW1,
    /// Interrupt triggered by hardware
    Hardware = IM_HW,
}

/// The exception cause code in cop0 r13
#[derive(Debug)]
pub enum Excode {
    /// Exception was caused by an interrupt
    Interrupt,
    /// Exception was caused by a syscall
    Syscall,
    /// Exception was caused by loading from a misaligned address
    AddressErrorLoad,
    /// Exception was caused by storing to a misaligned address
    AddressErrorStore,
    /// Exception was caused by executing an illegal instruction
    ReservedInstruction,
    /// Exception was caused by something else
    Other,
}

define_cop! {
    /// Breakpoint on execute register
    BPC<u32>; COP: 0; R: 3,
    /// Breakpoint on data access register
    BDA<u32>; COP: 0; R: 5,
    /// Breakpoint control register
    DCIC<u32>; COP: 0; R: 7,
    /// Bad virtual address
    BadVaddr<u32>; COP: 0; R: 8,
    /// Data access breakpoint mask
    BDAM<u32>; COP: 0; R: 9,
    /// Execute breakpoint mask
    BPCM<u32>; COP: 0; R: 11,
    /// Coprocessor system status register
    Status<u32>; COP: 0; R: 12,
    /// Exception cause register
    Cause<u32>; COP: 0; R: 13,
    /// Exception program counter
    EPC<u32>; COP: 0; R: 14,
    /// Processor ID register
    PRID<u32>; COP: 0; R: 15,
}

impl Debug for BadVaddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("cop0::BadVaddr")
            .field("bits", &self.to_bits())
            .finish()
    }
}
