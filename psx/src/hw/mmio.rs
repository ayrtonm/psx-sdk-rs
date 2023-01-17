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
