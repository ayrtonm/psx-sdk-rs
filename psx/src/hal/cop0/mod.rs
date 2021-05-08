use crate::hal::private;
use crate::hal::{MutRegister, Mutable, Read, Register, State, Write};
use core::marker::PhantomData;

mod status;

#[derive(Debug)]
pub enum Mode {
    Kernel = 0,
    User = 1,
}

pub use status::IntMask;

read_only_cop! {
    /// cop0r14     - EPC - Return Address from Trap
    EPC<u32>; COP: 0; R: 14
}

read_write_cop! {
    /// cop0r12     - SR - System status register
    Status<u32>; COP: 0; R: 12,
    /// cop0r13     - CAUSE - Describes the most recently recognised exception
    Cause<u32>; COP: 0; R: 13
}

// LLVM's assembler doesn't support the rfe instruction so this hack is required
// Also added a nop after the rfe in case jump delay slots apply
global_asm! {
    "return_from_exception:
    .word 0x00000010
    .word 0x00000000"
}

extern "C" {
    pub fn return_from_exception() -> !;
}
