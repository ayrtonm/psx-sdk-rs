use crate::hal::private;
use crate::hal::{Read, Register, Write, GEV};

read_only! {
    GlobalPointer<u32>
}

impl Read<u32> for GlobalPointer {
    fn read(&self) -> u32 {
        let gp;
        unsafe {
            asm!("move $2, $28", out("$2") gp);
        }
        gp
    }
}

impl GEV {
    pub fn install_handler(&mut self, f: fn()) {
        let addr = f as u32;
        let jal = (0b10 << 26) | ((addr & 0x03F_FFFF) >> 2);
        self.write(jal)
    }
}
