use bitflags::bitflags;

bitflags! {
    pub struct Status: u32 {
        const IEC = 1;
        const KUC = 1 << 1;

        const IEP = 1 << 2;
        const KUP = 1 << 3;

        const IEO = 1 << 4;
        const KUO = 1 << 5;

        const IM = 0x0000_FF00;
        const ISC = 1 << 16;
        const SWC = 1 << 17;
        const PZ = 1 << 18;
        const CM = 1 << 19;
        const PE = 1 << 20;
        const TS = 1 << 21;
        const BEV = 1 << 22;
        //const RE = 1 << 25;
        const CU0 = 1 << 28;
        //const CU1 = 1 << 29;
        const CU2 = 1 << 30;
        //const CU3 = 1 << 31;
    }
}

bitflags! {
    pub struct Cause: u32 {
        const EXCODE = 0x0000_007B;
        const IP = 0x0000_FF00;
        const CE = 0x3000_0000;
        const BD = 1 << 31;
    }
}

impl Status {
    pub fn read() -> Self {
        let status;
        unsafe {
            asm!("mfc0 $2, $12", out("$2") status);
            Status::from_bits_unchecked(status)
        }
    }

    pub fn write(self) {
        unsafe {
            asm!("mtc0 $2, $12", in("$2") self.bits());
        }
    }
}

impl Cause {
    pub fn read() -> Self {
        let cause;
        unsafe {
            asm!("mfc0 $2, $13", out("$2") cause);
            Cause::from_bits_unchecked(cause)
        }
    }

    pub fn write(self) {
        unsafe {
            asm!("mtc0 $2, $13", in("$2") self.bits());
        }
    }
}
