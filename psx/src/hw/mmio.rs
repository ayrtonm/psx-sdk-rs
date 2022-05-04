//! Memory-mapped IO definitions
use crate::hw::private::Primitive;
use crate::hw::Register;
use core::ptr::{read_volatile, write_volatile};

/// A memory register.
#[derive(Debug)]
pub struct MemRegister<T: Primitive, const ADDRESS: u32> {
    value: T,
}

impl<T: Primitive, const ADDRESS: u32> AsRef<T> for MemRegister<T, ADDRESS> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T: Primitive, const ADDRESS: u32> AsMut<T> for MemRegister<T, ADDRESS> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T: Primitive, const ADDRESS: u32> Register<T> for MemRegister<T, ADDRESS> {
    fn skip_load() -> Self {
        Self { value: T::from(0) }
    }

    fn load(&mut self) -> &mut Self {
        self.value = unsafe { read_volatile(ADDRESS as *const T) };
        self
    }

    fn store(&mut self) -> &mut Self {
        unsafe { write_volatile(ADDRESS as *mut T, self.value) }
        self
    }
}

/// Timer 0 Dotclock current counter value
pub type T0_CNT = MemRegister<u16, 0x1F80_1100>;
/// Timer 0 Dotclock counter mode
pub type T0_MODE = MemRegister<u16, 0x1F80_1104>;
/// Timer 0 Dotclock counter target value
pub type T0_TGT = MemRegister<u16, 0x1F80_1108>;
/// Timer 1 Horizontal Retrace current counter value
pub type T1_CNT = MemRegister<u16, 0x1F80_1110>;
/// Timer 1 Horizontal Retrace counter mode
pub type T1_MODE = MemRegister<u16, 0x1F80_1114>;
/// Timer 1 Horizontal Retrace counter target value
pub type T1_TGT = MemRegister<u16, 0x1F80_1118>;
/// Timer 2 1/8 system clock current counter value
pub type T2_CNT = MemRegister<u16, 0x1F80_1120>;
/// Timer 2 1/8 system clock counter mode
pub type T2_MODE = MemRegister<u16, 0x1F80_1124>;
/// Timer 2 1/8 system clock counter target value
pub type T2_TGT = MemRegister<u16, 0x1F80_1128>;
