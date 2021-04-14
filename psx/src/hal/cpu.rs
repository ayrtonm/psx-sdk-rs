use crate::hal::private;
use crate::hal::{Read, Register};

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
