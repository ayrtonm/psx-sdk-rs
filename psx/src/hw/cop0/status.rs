#![allow(dead_code)]

use crate::hw::cop0::{IntMask, Mode, Status};
use crate::hw::Register;
use core::fmt;
use core::fmt::{Debug, Formatter};

// Current interrupt enable
pub const IEC: u32 = 0;
// Current kernel/user mode
pub const KUC: u32 = 1;
// Previous interrupt enable
pub const IEP: u32 = 2;
// Previous kernel/user mode
pub const KUP: u32 = 3;
// Old interrupt enable
pub const IEO: u32 = 4;
// Old kernel/user mode
pub const KUO: u32 = 5;
// Software interrupt 0
pub const IM_SW0: u32 = 8;
// Software interrupt 1
pub const IM_SW1: u32 = 9;
// Hardware interrupt
pub const IM_HW: u32 = 10;
// Isolate cache
pub const ISC: u32 = 16;
// COP0 enable
pub const CU0: u32 = 28;
// COP2 enable
pub const CU2: u32 = 30;

impl Status {
    /// Checks if interrupts are allowed to cause exceptions.
    pub fn interrupts_enabled(&self) -> bool {
        self.all_set(1 << IEC)
    }

    /// Checks the privilege mode.
    pub fn get_mode(&self) -> Mode {
        if self.all_set(1 << KUC) {
            Mode::User
        } else {
            Mode::Kernel
        }
    }

    /// Checks if individual interrupt sources are allowed to cause exceptions.
    pub fn interrupt_masked(&self, int_mask: IntMask) -> bool {
        self.all_clear(1 << (int_mask as u32))
    }

    /// Checks if coprocessor 0 is enabled in user-mode.
    pub fn user_cop0_enabled(&self) -> bool {
        self.all_set(1 << CU0)
    }

    /// Checks if the GTE (coprocessor 2) is enabled.
    pub fn gte_enabled(&self) -> bool {
        self.all_set(1 << CU2)
    }

    /// Allows interrupts to cause exceptions.
    pub fn enable_interrupts(&mut self) -> &mut Self {
        self.set_bits(1 << IEC)
    }

    /// Forbids interrupts from causing exceptions.
    pub fn disable_interrupts(&mut self) -> &mut Self {
        self.clear_bits(1 << IEC)
    }

    /// Sets the privilege mode.
    pub fn set_mode(&mut self, mode: Mode) -> &mut Self {
        self.clear_bits(1 << KUC).set_bits((mode as u32) << KUC)
    }

    /// Masks an interrupt source, forbidding it from causing exceptions.
    pub fn mask_interrupt(&mut self, int_mask: IntMask) -> &mut Self {
        self.clear_bits(1 << (int_mask as u32))
    }

    /// Unmasks an interrupt source, allowing it to cause exceptions.
    pub fn unmask_interrupt(&mut self, int_mask: IntMask) -> &mut Self {
        self.set_bits(1 << (int_mask as u32))
    }

    /// Enables coprocessor 0 in user-mode.
    pub fn enable_user_cop0(&mut self) -> &mut Self {
        self.set_bits(1 << CU0)
    }

    /// Disables coprocessor 0 in user-mode.
    pub fn disable_user_cop0(&mut self) -> &mut Self {
        self.clear_bits(1 << CU0)
    }

    /// Enables the GTE.
    pub fn enable_gte(&mut self) -> &mut Self {
        self.set_bits(1 << CU2)
    }

    /// Disables the GTE.
    pub fn disable_gte(&mut self) -> &mut Self {
        self.clear_bits(1 << CU2)
    }
}

impl Debug for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("cop0::Status")
            .field("bits", &self.to_bits())
            .field("interrupts_enabled", &self.interrupts_enabled())
            .field("mode", &self.get_mode())
            .field(
                "hw_interrupt_masked",
                &self.interrupt_masked(IntMask::Hardware),
            )
            .field(
                "sw0_interrupt_masked",
                &self.interrupt_masked(IntMask::Software0),
            )
            .field(
                "sw1_interrupt_masked",
                &self.interrupt_masked(IntMask::Software1),
            )
            .field("user_cop0_enabled", &self.user_cop0_enabled())
            .field("gte_enabled", &self.gte_enabled())
            .finish()
    }
}
