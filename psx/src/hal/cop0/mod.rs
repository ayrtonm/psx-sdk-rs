use crate::hal::private;
use crate::hal::{MutRegister, Mutable, Read, Register, State, Write};
use core::marker::PhantomData;

mod status;

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
