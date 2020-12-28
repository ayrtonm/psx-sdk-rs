// Allowed to match nomenclature in [nocash specs](http://problemkaputt.de/psx-spx.htm).
#![allow(non_upper_case_globals)]

use crate::value::{Load, LoadMut, MutValue, Read, Value, Write};

/// System status register ([cop0r12](http://problemkaputt.de/psx-spx.htm#cop0registersummary))
pub struct Status;

/// Exception cause register ([cop0r13](http://problemkaputt.de/psx-spx.htm#cop0registersummary))
pub struct Cause;

/// Exception program counter register ([cop0r14](http://problemkaputt.de/psx-spx.htm#cop0registersummary))
pub struct EPC;

impl Read<u32> for Status {
    #[inline(always)]
    unsafe fn read(&self) -> u32 {
        let status;
        asm!("mfc0 $2, $12", out("$2") status);
        status
    }
}

impl Write<u32> for Status {
    #[inline(always)]
    unsafe fn write(&mut self, status: u32) {
        asm!("mtc0 $2, $12", in("$2") status)
    }
}

impl LoadMut<u32> for Status {}

impl Status {
    /// Current Interrupt Enable
    pub const IEc: u32 = 1;
    /// Current Kernel/User Mode
    pub const KUc: u32 = 1 << 1;
    /// Previous Interrupt Enable
    pub const IEp: u32 = 1 << 2;
    /// Previous Kernel/User Mode
    pub const KUp: u32 = 1 << 3;
    /// Old Interrupt Enable
    pub const IEo: u32 = 1 << 4;
    /// Old Kernel/User Mode
    pub const KUo: u32 = 1 << 5;

    /// Mask bit for software interrupt 1. When set software interrupt 1 is
    /// allowed to cause an exception.
    pub const ImSw1: u32 = 1 << 8;
    /// Mask bit for software interrupt 2. When set software interrupt 2 is
    /// allowed to cause an exception.
    pub const ImSw2: u32 = 1 << 9;
    /// Mask bit for hardware interrupts. When set peripherals are allowed to
    /// cause an exception.
    pub const ImHw: u32 = 1 << 10;

    /// Cache isolated. When isolated, all load and store operations are
    /// targetted to the data cache and never the main memory.
    pub const Isc: u32 = 1 << 16;
    /// Caches swapped. When swapped the instruction cache becomes the data
    /// cache and vice versa.
    pub const Swc: u32 = 1 << 17;

    /// Boot exception vectors in RAM/ROM (0=RAM/KSEG0, 1=ROM/KSEG1)
    pub const BEV: u32 = 1 << 22;
    /// Coprocessor 0 enabled in user mode
    pub const CU0: u32 = 1 << 28;
    /// Graphics transformation engine (Coprocessor 2) enabled.
    pub const CU2: u32 = 1 << 30;
}

/// A [`Value`] for [`Status`].
pub type StatusValue<'r> = Value<'r, u32, Status>;
/// A [`MutValue`] for [`Status`].
pub type StatusMutValue<'r> = MutValue<'r, u32, Status>;

impl StatusValue<'_> {
    /// Checks if interrupts are enabled.
    #[inline(always)]
    pub fn interrupts_enabled(&self) -> bool {
        self.contains(Status::IEc)
    }
}

impl StatusMutValue<'_> {
    /// Enters a critical section.
    #[inline(always)]
    pub fn enter_critical_section(self) -> Self {
        self.set(Status::IEp | Status::ImHw)
    }

    /// Exits a critical section.
    #[inline(always)]
    pub fn exit_critical_section(self) -> Self {
        self.clear(Status::IEp | Status::ImHw)
    }

    /// Enables interrupts in `Status`.
    #[inline(always)]
    pub fn enable_interrupts(self) -> Self {
        self.set(Status::IEc)
    }

    /// Disables interrupts in `Status`.
    #[inline(always)]
    pub fn disable_interrupts(self) -> Self {
        self.clear(Status::IEc)
    }
}

impl Read<u32> for Cause {
    #[inline(always)]
    unsafe fn read(&self) -> u32 {
        let cause;
        asm!("mfc0 $2, $13", out("$2") cause);
        cause
    }
}

impl Write<u32> for Cause {
    #[inline(always)]
    unsafe fn write(&mut self, cause: u32) {
        asm!("mtc0 $2, $13", in("$2") cause)
    }
}

impl LoadMut<u32> for Cause {}

impl Cause {
    /// Pending bit for software interrupt 1 (R/W). Causes an interrupt when set
    /// if [`Status::ImSw1`] is set in the status register.
    pub const ImSw1: u32 = 1 << 8;
    /// Pending bit for software interrupt 2 (R/W). Causes an interrupt when set
    /// if [`Status::ImSw2`] is set in the status register.
    pub const ImSw2: u32 = 1 << 9;
    /// Pending bit for hardware interrupts (R). Causes an interrupt when set if
    /// [`Status::ImHw`] is set in the status register.
    pub const ImHw: u32 = 1 << 10;
}

impl Read<u32> for EPC {
    #[inline(always)]
    unsafe fn read(&self) -> u32 {
        let epc;
        asm!("mfc0 $2, $14", out("$2") epc);
        epc
    }
}

impl Load<u32> for EPC {}

/// Executes the coprocessor RFE instruction ([cop0cmd=10h](http://problemkaputt.de/psx-spx.htm#cop0exceptionhandling)).
/// This prepares for a return from an exception.
#[inline(always)]
pub fn rfe() {
    unsafe {
        asm!(".word 0x42000010");
    }
}
