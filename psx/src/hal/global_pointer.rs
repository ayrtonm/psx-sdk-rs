use crate::hal::private;
use crate::hal::{Read, Register};

#[derive(PartialEq, Eq)]
pub struct GlobalPointer(u32);

impl GlobalPointer {
    pub unsafe fn from_bits(bits: u32) -> Self {
        Self(bits)
    }
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

impl private::HasValue<u32> for GlobalPointer {
    fn get(&self) -> u32 {
        self.0
    }

    fn get_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}

impl Register<u32> for GlobalPointer {
    fn load() -> Self {
        let unread = Self(0);
        Self(unread.read())
    }
}
