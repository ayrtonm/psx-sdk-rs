use crate::hal::private;
use crate::hal::{MutRegister, Mutable, Read, Register, State, Write};
use core::marker::PhantomData;

read_write! {
    /// cop2r30 - LZCS - Count Leading Bits Source data
    LZCS<u32>
}

read_only! {
    /// cop2r31 - LZCR - Count Leading Bits Result
    LZCR<u32>
}

impl<S: State> Read<u32> for LZCS<S> {
    fn read(&self) -> u32 {
        let lzcs;
        unsafe {
            asm!("mfc2 $2, $30", out("$2") lzcs);
        }
        lzcs
    }
}

impl Write<u32> for LZCS<Mutable> {
    fn write(&mut self, lzcs: u32) {
        unsafe {
            asm!("mtc2 $2, $30", in("$2") lzcs);
        }
    }
}

impl Read<u32> for LZCR {
    fn read(&self) -> u32 {
        let lzcr;
        unsafe {
            asm!("mfc2 $2, $30", out("$2") lzcr);
        }
        lzcr
    }
}
