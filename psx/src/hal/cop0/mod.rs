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
    /// r8/BadVaddr - Bad virtual address
    BadVaddr<u32>; COP: 0; R: 8,
    /// r14/EPC - Return address from trap
    EPC<u32>; COP: 0; R: 14,
    /// r15/PRID - Processor ID
    PRID<u32>; COP: 0; R: 15
}

read_write_cop! {
    /// r3/BPC - Breakpoint on execute
    BPC<u32>; COP: 0; R: 3,
    /// r5/BDA - Breakpoint on data access
    BDA<u32>; COP: 0; R: 5,
    /// r7/DCIC - Breakpoint control
    DCIC<u32>; COP: 0; R: 7,
    /// r9/BDAM - Breakpoint on data access mask
    BDAM<u32>; COP: 0; R: 9,
    /// r11/BPCM - Execute breakpoint mask
    BPCM<u32>; COP: 0; R: 11,
    /// r12/SR - System status register
    Status<u32>; COP: 0; R: 12,
    /// r13/CAUSE - Describes the most recently recognised exception
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
