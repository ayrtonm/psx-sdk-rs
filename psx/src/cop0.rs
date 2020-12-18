// TODO: It'd be nice to have some safeguard against implementing write methods
// to cop registers TODO: Dead code is temporarily allowed while I make the flag
// methods
#![allow(dead_code)]
#[must_use = "Modifications to COP0 status must be written back to cop0r12"]
pub struct Status(u32);
pub struct Cause(u32);
pub struct EPC;

impl Status {
    const IEC: u32 = 1;
    const KUC: u32 = 1 << 1;
    const IEP: u32 = 1 << 2;
    const KUP: u32 = 1 << 3;
    const IEO: u32 = 1 << 4;
    const KUO: u32 = 1 << 5;

    const IM_SW1: u32 = 1 << 8;
    const IM_SW2: u32 = 1 << 9;
    const IM_HW: u32 = 1 << 10;
    //const IM = 0x0000_FF00;
    // Bits 11-15 of IM are always zero

    const ISC: u32 = 1 << 16;
    const SWC: u32 = 1 << 17;
    const PZ: u32 = 1 << 18;
    const CM: u32 = 1 << 19;
    const PE: u32 = 1 << 20;
    const TS: u32 = 1 << 21;
    const BEV: u32 = 1 << 22;
    //const RE = 1 << 25;
    const CU0: u32 = 1 << 28;
    //const CU1 = 1 << 29;
    pub const CU2: u32 = 1 << 30;
    //const CU3 = 1 << 31;

    #[inline(always)]
    pub fn read() -> Self {
        let status;
        unsafe {
            asm!("mfc0 $2, $12", out("$2") status);
        }
        Self(status)
    }

    #[inline(always)]
    pub fn write(self) {
        unsafe {
            asm!("mtc0 $2, $12", in("$2") self.0);
        }
    }

    #[inline(always)]
    pub fn contains(&self, flags: u32) -> bool {
        self.0 & flags != 0
    }

    #[inline(always)]
    pub fn set(mut self, flags: u32) -> Self {
        self.0 |= flags;
        self
    }

    #[inline(always)]
    pub fn clear(mut self, flags: u32) -> Self {
        self.0 &= !flags;
        self
    }

    #[inline(always)]
    pub fn interrupts_enabled(&self) -> bool {
        self.contains(Self::IEC)
    }

    // TODO: disable_interrupts and enable_interrupts might not be complete
    #[inline(always)]
    pub fn disable_interrupts(self) -> Self {
        self.clear(Self::IEC)
    }

    #[inline(always)]
    pub fn enable_interrupts(self) -> Self {
        self.set(Self::IEC)
    }
}

impl Cause {
    const EXCODE: u32 = 0x0000_007B;
    const IP: u32 = 0x0000_FF00;
    const CE: u32 = 0x3000_0000;
    const BD: u32 = 1 << 31;

    #[inline(always)]
    pub fn read() -> Self {
        let cause;
        unsafe {
            asm!("mfc0 $2, $13", out("$2") cause);
            Self(cause)
        }
    }
}

impl EPC {
    #[inline(always)]
    pub fn read() -> u32 {
        let epc;
        unsafe {
            asm!("mfc0 $2, $14", out("$2") epc);
        }
        epc
    }
}

//#[naked]
//#[inline(always)]
//pub fn rfe() {
//    unsafe {
//        asm!(".macro rfe
//              .word 0x42000010
//              .endmacro
//              rfe");
//    }
//}
