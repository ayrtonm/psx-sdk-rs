use crate::hal::private;
use crate::hal::{MutRegister, Mutable, Read, Register, State, Write};
use core::marker::PhantomData;

read_only! {
    EPC<u32>
}

read_write! {
    Status<u32>,
    Cause<u32>
}

impl<S: State> Read<u32> for Status<S> {
    fn read(&self) -> u32 {
        let status;
        unsafe {
            asm!("mfc0 $2, $12", out("$2") status);
        }
        status
    }
}

impl Write<u32> for Status<Mutable> {
    fn write(&mut self, status: u32) {
        unsafe {
            asm!("mtc0 $2, $12", in("$2") status);
        }
    }
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
