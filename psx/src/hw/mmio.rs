//! Memory-mapped IO definitions
use crate::hw::private::Primitive;
use crate::hw::Register;
use core::fmt;
use core::fmt::{Debug, Formatter};
use core::ptr::{read_volatile, write_volatile};

/// A memory register.
#[repr(C)]
pub struct MemRegister<T: Primitive, const ADDRESS: u32> {
    value: T,
}

impl<T: Primitive, const ADDRESS: u32> Debug for MemRegister<T, ADDRESS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("MemRegister")
            .field("bits", &self.to_bits())
            .finish()
    }
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

/// A [`u32`] memory register, split into two [`u16`]
#[repr(C)]
pub struct SplitU32MemRegister<const ADDRESS: u32> {
    value: u32,
}

impl<const ADDRESS: u32> Debug for SplitU32MemRegister<ADDRESS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("MemRegister")
            .field("bits", &self.to_bits())
            .finish()
    }
}

impl<const ADDRESS: u32> AsRef<u32> for SplitU32MemRegister<ADDRESS> {
    fn as_ref(&self) -> &u32 {
        &self.value
    }
}

impl<const ADDRESS: u32> AsMut<u32> for SplitU32MemRegister<ADDRESS> {
    fn as_mut(&mut self) -> &mut u32 {
        &mut self.value
    }
}

impl<const ADDRESS: u32> Register<u32> for SplitU32MemRegister<ADDRESS> {
    fn skip_load() -> Self {
        Self { value: 0 }
    }

    fn load(&mut self) -> &mut Self {
        let low_half = unsafe { read_volatile(ADDRESS as *const u16) } as u32;
        let upper_half = unsafe { read_volatile((ADDRESS + 2) as *const u16) } as u32;
        self.value = (upper_half << 16) | low_half;
        self
    }

    fn store(&mut self) -> &mut Self {
        unsafe { write_volatile(ADDRESS as *mut u16, (self.value & 0xFFFF) as u16) };
        unsafe { write_volatile((ADDRESS + 2) as *mut u16, (self.value >> 16) as u16) };
        self
    }
}
