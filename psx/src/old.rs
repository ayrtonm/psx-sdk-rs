///
/// Methods for modifying the system status register. After loading or modifying
/// a `Status`, it must be written back to coprocessor 0 or it will produce a
/// compile-time warning. For example:
///
/// ```
///     let status = cop0::Status::load_mut().clear(..);
///     status.set(..).store();
/// ```
///
/// To intentionally load and modify the status register without storing it, the
/// following idiom can be used.
///
/// ```
///     let _ = cop0::Status::load_mut().set(..);
/// ```
///
/// Note that in this scenario, while the load from cop0 will always occur, any
/// modifications to the loaded value are very likely to be optimized out by the
/// compiler.
pub struct Status;

/// Exception cause register  ([cop0r13](http://problemkaputt.de/psx-spx.htm#cop0registersummary))
pub struct Cause(u32);

/// Exception program counter register ([cop0r14](http://problemkaputt.de/psx-spx.htm#cop0registersummary))
pub struct EPC(u32);

impl Status {

    /// Loads the `Status` register from coprocessor 0.
    pub fn load_mut() -> MutValue<Self, u32> {
    }

    /// Stores the `Status` register in coprocessor 0.
    #[inline(always)]
    pub fn store(self) {
        unsafe {
            asm!("mtc0 $2, $12", in("$2") self.0);
        }
    }

}

impl Value<Status, u32> {
}

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
    /// Loads the `Cause` register from coprocessor 0.
    #[inline(always)]
    pub fn load_mut() -> Self {
        let cause;
        unsafe {
            asm!("mfc0 $2, $13", out("$2") cause);
        }
        Self(cause)
    }

    /// Stores the `Cause` register in coprocessor 0.
    #[inline(always)]
    pub fn store(self) {
        unsafe {
            asm!("mtc0 $2, $13", in("$2") self.0);
        }
    }
}

impl EPC {
    /// Loads the `EPC` register from coprocessor 0.
    #[inline(always)]
    pub fn load() -> Self {
        let epc;
        unsafe {
            asm!("mfc0 $2, $14", out("$2") epc);
        }
        Self(epc)
    }
}

/// Executes the coprocessor RFE instruction ([cop0cmd=10h](http://problemkaputt.de/psx-spx.htm#cop0exceptionhandling)). This prepares for a return from an exception.
#[inline(always)]
pub fn rfe() {
    unsafe {
        asm!(".word 0x42000010");
    }
}
