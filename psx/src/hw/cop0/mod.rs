//! System Control Coprocessor
//!
//! This module provides access to registers and instructions in the system
//! control coprocessor, cop0.

use crate::hw::cop0::status::{IM_HW, IM_SW0, IM_SW1};
use crate::hw::Register;

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

/// Interrupt mask for determining which interrupts are allowed to cause
/// exceptions.
#[repr(u32)]
pub enum IntMask {
    /// One of the software-based interrupts
    Software0 = IM_SW0,
    /// One of the software-based interrupts
    Software1 = IM_SW1,
    /// Interrupt triggered by hardware
    Hardware = IM_HW,
}

/// A handle to a coprocessor register
pub struct CopRegister<const COP: u32, const REG: u32> {
    value: u32,
}

impl<const COP: u32, const REG: u32> AsRef<u32> for CopRegister<COP, REG> {
    fn as_ref(&self) -> &u32 {
        &self.value
    }
}

impl<const COP: u32, const REG: u32> AsMut<u32> for CopRegister<COP, REG> {
    fn as_mut(&mut self) -> &mut u32 {
        &mut self.value
    }
}

macro_rules! define_cop0 {
    ($(#[$($meta:meta)*])* $name:ident, $reg:expr) => {
        $(#[$($meta)*])*
        pub type $name = CopRegister<0, $reg>;
        define_cop!(0, $reg);
    };
}

define_cop0! {
    /// Breakpoint on execute register
    BPC, 3
}
define_cop0! {
    /// Breakpoint on data access register
    BDA, 5
}
define_cop0! {
    /// Breakpoint control register
    DCIC, 7
}
define_cop0! {
    /// Bad virtual address
    BadVaddr, 8
}
define_cop0! {
    /// Data access breakpoint mask
    BDAM, 9
}
define_cop0! {
    /// Execute breakpoint mask
    BPCM, 11
}
define_cop0! {
    /// Coprocessor system status register
    Status, 12
}
define_cop0! {
    /// Exception cause register
    Cause, 13
}
define_cop0! {
    /// Exception program counter
    EPC, 14
}
define_cop0! {
    /// Processor ID register
    PRID, 15
}
