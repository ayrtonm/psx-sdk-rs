use super::ty::Name;
use crate::hal::{MutRegister, Mutable, Register, State, DPCR};

impl<S: State> DPCR<S> {
    pub fn enabled(&self, ch: Name) -> bool {
        self.all_set(enable_bit(ch))
    }
}

impl DPCR<Mutable> {
    pub fn enable(&mut self, ch: Name) -> &mut Self {
        self.set_bits(enable_bit(ch))
    }

    pub fn disable(&mut self, ch: Name) -> &mut Self {
        self.clear_bits(enable_bit(ch))
    }

    pub fn enable_all(&mut self) -> &mut Self {
        self.set_bits(ENABLE_BITS)
    }

    pub fn disable_all(&mut self) -> &mut Self {
        self.clear_bits(ENABLE_BITS)
    }
}

const fn enable_bit(ch: Name) -> u32 {
    let bit = (ch as u32 * 4) + 3;
    1 << bit
}

const ENABLE_BITS: u32 = {
    enable_bit(Name::MDECIn) |
        enable_bit(Name::MDECOut) |
        enable_bit(Name::GPU) |
        enable_bit(Name::CDROM) |
        enable_bit(Name::SPU) |
        enable_bit(Name::PIO) |
        enable_bit(Name::OTC)
};
