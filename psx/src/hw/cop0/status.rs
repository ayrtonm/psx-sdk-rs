use crate::hw::cop0::{IntSrc, Mode, Status};
use crate::hw::Register;
use crate::CriticalSection;
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
// Swap cache
pub const SWC: u32 = 17;
// Boot (ROM) exception vector
pub const BEV: u32 = 22;
// COP0 enable
pub const CU0: u32 = 28;
// COP2 enable
pub const CU2: u32 = 30;

impl Status {
    /// Checks if interrupts are allowed to cause exceptions.
    pub fn interrupts_enabled(&self) -> bool {
        self.all_set(1 << IEC)
    }

    /// Checks if interrupts are not allowed to cause exceptions.
    pub fn interrupts_disabled(&self) -> bool {
        self.all_clear(1 << IEC)
    }

    /// Checks the privilege mode.
    pub fn get_mode(&self) -> Mode {
        if self.all_set(1 << KUC) {
            Mode::User
        } else {
            Mode::Kernel
        }
    }

    /// Checks if interrupts were enabled before the current exception.
    pub fn previous_interrupt_enabled(&self) -> bool {
        self.all_set(1 << IEP)
    }

    /// Checks if interrupts were disabled before the current exception.
    pub fn previous_interrupt_disabled(&self) -> bool {
        self.all_clear(1 << IEP)
    }

    /// Checks the privilege mode before the current exception.
    pub fn previous_mode(&self) -> Mode {
        if self.all_set(1 << KUP) {
            Mode::User
        } else {
            Mode::Kernel
        }
    }

    /// Checks if interrupts were enabled two exceptions ago.
    pub fn old_interrupt_enabled(&self) -> bool {
        self.all_set(1 << IEO)
    }

    /// Checks the privilege mode two exceptions ago.
    pub fn old_mode(&self) -> Mode {
        if self.all_set(1 << KUO) {
            Mode::User
        } else {
            Mode::Kernel
        }
    }

    /// Checks if individual interrupt sources are allowed to cause exceptions.
    pub fn interrupt_unmasked(&self, int_src: IntSrc) -> bool {
        self.all_set(1 << (int_src as u32))
    }

    /// Checks if individual interrupt sources are forbidden from causing
    /// exceptions.
    pub fn interrupt_masked(&self, int_src: IntSrc) -> bool {
        !self.interrupt_unmasked(int_src)
    }

    /// Checks if the boot (ROM) exception vectors are in use.
    pub fn using_boot_vectors(&self) -> bool {
        self.all_set(1 << BEV)
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

    /// Set interrupts previously enabled flag.
    pub fn previous_interrupt_enable(&mut self) -> &mut Self {
        self.set_bits(1 << IEP)
    }

    /// Clear interrupts previously enabled flag.
    pub fn previous_interrupt_disable(&mut self) -> &mut Self {
        self.clear_bits(1 << IEP)
    }

    /// Sets the privilege mode.
    pub fn set_mode(&mut self, mode: Mode) -> &mut Self {
        self.clear_bits(1 << KUC).set_bits((mode as u32) << KUC)
    }

    /// Masks an interrupt source, forbidding it from causing exceptions.
    pub fn mask_interrupt(&mut self, int_src: IntSrc) -> &mut Self {
        self.clear_bits(1 << (int_src as u32))
    }

    /// Unmasks an interrupt source, allowing it to cause exceptions.
    pub fn unmask_interrupt(&mut self, int_src: IntSrc) -> &mut Self {
        self.set_bits(1 << (int_src as u32))
    }

    /// Checks if the data cache is isolated from memory.
    pub fn cache_isolated(&self) -> bool {
        self.all_set(1 << ISC)
    }

    /// Isolates the data cache from main memory.
    ///
    /// This causes stores targetting the data cache to modify it without
    /// writing to main memory. Loads return the data value from the cache
    /// whether or not a cache hit occurred.
    pub fn isolate_cache(&mut self, isolate: bool) -> &mut Self {
        if isolate {
            self.set_bits(1 << ISC)
        } else {
            self.clear_bits(1 << ISC)
        }
    }

    /// Checks if the data cache and instruction cache are swapped.
    pub fn cache_swapped(&self) -> bool {
        self.all_set(1 << SWC)
    }

    /// Swaps the data cache and instruction cache, invalidating all
    /// intstruction cache entries.
    pub fn swap_cache(&mut self) -> &mut Self {
        self.toggle_bits(1 << SWC)
    }

    /// Sets the status register to use the boot (ROM) exception vectors.
    pub fn use_boot_vectors(&mut self, rom: bool) -> &mut Self {
        if rom {
            self.set_bits(1 << BEV)
        } else {
            self.clear_bits(1 << BEV)
        }
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

    /// Run a closure in an interrupt-free context
    pub fn critical_section<F: FnMut(&mut CriticalSection) -> R, R>(&mut self, mut f: F) -> R {
        let in_critical_section =
            self.interrupt_masked(IntSrc::Hardware) || self.interrupts_disabled();

        if !in_critical_section {
            self.mask_interrupt(IntSrc::Hardware)
                .disable_interrupts()
                .store();
        }
        // SAFETY: We are in a critical section so we can create this
        let mut cs = unsafe { CriticalSection::new() };
        let res = f(&mut cs);
        if !in_critical_section {
            self.unmask_interrupt(IntSrc::Hardware)
                .enable_interrupts()
                .store();
        }
        res
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
                &self.interrupt_masked(IntSrc::Hardware),
            )
            .field(
                "sw0_interrupt_masked",
                &self.interrupt_masked(IntSrc::Software0),
            )
            .field(
                "sw1_interrupt_masked",
                &self.interrupt_masked(IntSrc::Software1),
            )
            .field("boot_vectors", &self.using_boot_vectors())
            .field("user_cop0_enabled", &self.user_cop0_enabled())
            .field("gte_enabled", &self.gte_enabled())
            .finish()
    }
}
