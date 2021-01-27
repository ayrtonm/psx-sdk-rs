use crate::dma::ChannelName;
use crate::hal::{MutRegister, Mutable, Register, State, DPCR};

impl<S: State> DPCR<S> {
    pub fn enabled(&self, ch: ChannelName) -> bool {
        self.contains(enable_bit(ch))
    }
}

impl DPCR<Mutable> {
    pub fn enable(&mut self, ch: ChannelName) -> &mut Self {
        self.set(enable_bit(ch))
    }

    pub fn disable(&mut self, ch: ChannelName) -> &mut Self {
        self.clear(enable_bit(ch))
    }

    pub fn enable_all(&mut self) -> &mut Self {
        self.set(ENABLE_BITS)
    }

    pub fn disable_all(&mut self) -> &mut Self {
        self.clear(ENABLE_BITS)
    }
}

const fn enable_bit(ch: ChannelName) -> u32 {
    let bit = (ch as u32 * 4) + 3;
    1 << bit
}

const ENABLE_BITS: u32 = {
    enable_bit(ChannelName::MDECIn) |
        enable_bit(ChannelName::MDECOut) |
        enable_bit(ChannelName::GPU) |
        enable_bit(ChannelName::CDROM) |
        enable_bit(ChannelName::SPU) |
        enable_bit(ChannelName::PIO) |
        enable_bit(ChannelName::OTC)
};
