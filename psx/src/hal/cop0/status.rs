use super::{Mode, Status};
use crate::hal::{MutRegister, Mutable, Register, State};

const IEC: u32 = 0;
const KUC: u32 = 1;
const IEP: u32 = 2;
const KUP: u32 = 3;
const IEO: u32 = 4;
const KUO: u32 = 5;
const IM_SW0: u32 = 8;
const IM_SW1: u32 = 9;
const IM_HW: u32 = 10;
const ISC: u32 = 16;
const CU0: u32 = 28;
const CU2: u32 = 30;

// TODO: Is it worth representing bits 8-9? i.e. is there some use for software
// ints (maybe in callbacks)?
#[repr(u32)]
pub enum IntMask {
    Software0 = IM_SW0,
    Software1 = IM_SW1,
    Hardware = IM_HW,
}

impl<S: State> Status<S> {
    pub fn ints_enabled(&self) -> bool {
        self.all_set(1 << IEC)
    }

    pub fn get_mode(&self) -> Mode {
        if self.all_set(1 << KUC) {
            Mode::User
        } else {
            Mode::Kernel
        }
    }

    pub fn int_masked(&self, int: IntMask) -> bool {
        self.all_set(1 << (int as u32))
    }

    pub fn user_cop0_enabled(&self) -> bool {
        self.all_set(1 << CU0)
    }

    pub fn gte_enabled(&self) -> bool {
        self.all_set(1 << CU2)
    }
}

impl Status<Mutable> {
    pub fn enable_ints(&mut self, enable: bool) -> &mut Self {
        self.set_bits((enable as u32) << IEC)
    }

    pub fn set_mode(&mut self, mode: Mode) -> &mut Self {
        self.set_bits((mode as u32) << KUC)
    }

    pub fn mask_int(&mut self, int: IntMask, enable: bool) -> &mut Self {
        self.set_bits((enable as u32) << (int as u32))
    }

    pub fn enable_user_cop0(&mut self, enable: bool) -> &mut Self {
        self.set_bits((enable as u32) << CU0)
    }

    pub fn enable_gte(&mut self, enable: bool) -> &mut Self {
        self.set_bits((enable as u32) << CU2)
    }
}
