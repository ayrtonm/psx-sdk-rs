use crate::hw::cdrom::{Port, Status};
use crate::hw::Register;
use core::hint::unreachable_unchecked;

const XA_ADPCM: u8 = 2;
const PRMEMPT: u8 = 3;
const PRMWRDY: u8 = 4;
const RSLRRDY: u8 = 5;
const DRQSTS: u8 = 6;
const BUSYSTS: u8 = 7;

// TODO: Add a better Debug impl for this
impl Status {
    pub fn get_port(&self) -> Port {
        match self.to_bits() & 0b11 {
            0 => Port::Port0,
            1 => Port::Port1,
            2 => Port::Port2,
            3 => Port::Port3,
            // SAFETY: & 0b11 ensures this cannot be greater than 3
            _ => unsafe { unreachable_unchecked() },
        }
    }

    pub fn set_port(&mut self, port: Port) -> &mut Self {
        self.clear_bits(0b11).set_bits(port as u8)
    }

    pub fn xa_adpcm_empty(&self) -> bool {
        self.all_clear(1 << XA_ADPCM)
    }

    pub fn param_fifo_empty(&self) -> bool {
        self.all_set(1 << PRMEMPT)
    }

    pub fn param_fifo_full(&self) -> bool {
        self.all_clear(1 << PRMWRDY)
    }

    pub fn response_fifo_empty(&self) -> bool {
        self.all_clear(1 << RSLRRDY)
    }

    pub fn data_fifo_empty(&self) -> bool {
        self.all_clear(1 << DRQSTS)
    }

    pub fn busy(&self) -> bool {
        self.all_set(1 << BUSYSTS)
    }
}
