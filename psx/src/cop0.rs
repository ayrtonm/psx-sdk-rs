use crate::value::{Load, MutValue, Store, Value};

/// System status register ([cop0r12](http://problemkaputt.de/psx-spx.htm#cop0registersummary))
pub struct Status;

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

    /// Loads the `Status` register from coprocessor 0, immutably borrowing the
    /// register.
    pub fn load(&self) -> Value<Self, u32> {
        Value::load(self)
    }

    /// Loads the `Status` register from coprocessor 0, exclusively borrowing
    /// the register.
    pub fn load_mut(&mut self) -> MutValue<Self, u32> {
        MutValue::load_mut(self)
    }
}

impl Load<u32> for Status {
    #[inline(always)]
    unsafe fn load(&self) -> u32 {
        let status;
        asm!("mfc0 $2, $12", out("$2") status);
        status
    }
}

impl Store<u32> for Status {
    #[inline(always)]
    unsafe fn store(&mut self, status: u32) {
        asm!("mtc0 $2, $12", in("$2") status)
    }
}

impl Value<Status, u32> {
    /// Checks if interrupts are enabled in the value loaded from the `Status`
    /// register.
    #[inline(always)]
    pub fn interrupts_enabled(&self) -> bool {
        self.bits & Status::IEc != 0
    }
}

impl MutValue<'_, Status, u32> {
    /// Sets the given `flags` in `Status`. This has no effect until `Status`
    /// is stored in coprocessor 0.
    #[inline(always)]
    pub fn set(mut self, flags: u32) -> Self {
        self.bits |= flags;
        self
    }

    /// Clears the given `flags` in `Status`. This has no effect until `Status`
    /// is stored in coprocessor 0.
    #[inline(always)]
    pub fn clear(mut self, flags: u32) -> Self {
        self.bits &= !flags;
        self
    }

    /// Enables interrupts in `Status`. This has no effect until `Status` is
    /// stored in coprocessor 0.
    #[inline(always)]
    pub fn enable_interrupts(mut self) -> Self {
        self.bits |= Status::IEc;
        self
    }

    /// Disables interrupts in `Status`. This has no effect until `Status` is
    /// stored in coprocessor 0.
    #[inline(always)]
    pub fn disable_interrupts(mut self) -> Self {
        self.bits &= !Status::IEc;
        self
    }
}
