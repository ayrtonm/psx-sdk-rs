use crate::hal::private;
use crate::hal::{MutRegister, Mutable, Read, Register, State, Write};
use core::marker::PhantomData;

read_write_cop! {
    /// cop2r30 - LZCS - Count Leading Bits Source data
    LZCS<u32>; COP: 2; R: 30
}

read_only_cop! {
    /// cop2r31 - LZCR - Count Leading Bits Result
    LZCR<u32>; COP: 2; R: 31
}

#[cfg(test)]
mod tests {
    use super::{LZCR, LZCS};
    use crate::hal::cop0;
    use crate::hal::{MutRegister, Register};
    #[test_case]
    fn leading_zero_count() {
        // Test case has three leading zeros
        let value = 0b0001_0000u32 << 24;
        cop0::Status::load().enable_gte(true).store();
        LZCS::skip_load().assign(value).store();
        // lzc is a special GTE function in that it doesn't halt the CPU when trying to
        // read incomplete results so we have to manually insert NOPs
        crate::timer::delay(150);
        let lzc = LZCR::load().bits();
        crate::println!("{}", lzc);
        assert!(lzc == 3);
    }
}
