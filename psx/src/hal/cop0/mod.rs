use crate::hal::private;
use crate::hal::{MutRegister, Mutable, Read, Register, State, Write};
use core::marker::PhantomData;

mod status;

pub enum Mode {
    Kernel = 0,
    User = 1,
}

read_only! {
    /// cop0r14     - EPC - Return Address from Trap
    EPC<u32>
}

read_write! {
    /// cop0r12     - SR - System status register
    Status<u32>,
    /// cop0r13     - CAUSE - Describes the most recently recognised exception
    Cause<u32>
}

impl<S: State> Read<u32> for Cause<S> {
    fn read(&self) -> u32 {
        let cause;
        unsafe {
            asm!("mfc0 $2, $13", out("$2") cause);
        }
        cause
    }
}

impl Write<u32> for Cause<Mutable> {
    fn write(&mut self, cause: u32) {
        unsafe {
            asm!("mtc0 $2, $13", in("$2") cause);
        }
    }
}

impl Read<u32> for EPC {
    fn read(&self) -> u32 {
        let epc;
        unsafe {
            asm!("mfc0 $2, $14", out("$2") epc);
        }
        epc
    }
}
