// Temporary
#![allow(dead_code)]

use super::{Mode, Status};
use crate::hal::{MutRegister, Mutable, Read, Register, State, Write};

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

impl<S: State> Status<S> {
    pub fn interrupts_enabled(&self) -> bool {
        self.all_set(1 << IEC)
    }

    pub fn get_mode(&self) -> Mode {
        if self.all_set(1 << KUC) {
            Mode::User
        } else {
            Mode::Kernel
        }
    }

    pub fn user_cop0_enabled(&self) -> bool {
        self.all_set(1 << CU0)
    }

    pub fn gte_enabled(&self) -> bool {
        self.all_set(1 << CU2)
    }
}

impl Status<Mutable> {
    pub fn enable_interrupts(&mut self, enable: bool) -> &mut Self {
        self.set_bits((enable as u32) << IEC)
    }

    pub fn set_mode(&mut self, mode: Mode) -> &mut Self {
        self.set_bits((mode as u32) << KUC)
    }

    pub fn enable_user_cop0(&mut self, enable: bool) -> &mut Self {
        self.set_bits((enable as u32) << CU0)
    }

    pub fn enable_gte(&mut self, enable: bool) -> &mut Self {
        self.set_bits((enable as u32) << CU2)
    }
}
