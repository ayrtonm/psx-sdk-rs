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
