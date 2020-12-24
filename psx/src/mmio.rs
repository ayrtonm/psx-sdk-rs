use core::ptr::{read_volatile, write_volatile};

use crate::value::{Load, LoadMut, Read, Write};

/// An addressable memory-mapped I/O register.
pub trait Address<T> {
    /// The memory address the register is mapped to.
    const ADDRESS: u32;
}

impl<T: Copy, R: Address<T> + Load<T>> Read<T> for R {
    /// Loads an I/O register from memory. Use sparingly as calls cannot be
    /// optimized out by the compiler.
    #[inline(always)]
    unsafe fn read(&self) -> T {
        read_volatile(Self::ADDRESS as *const T)
    }
}

impl<T: Copy, R: Address<T> + LoadMut<T>> Write<T> for R {
    /// Stores an I/O register in memory. Use sparingly as calls cannot be
    /// optimized out by the compiler.
    #[inline(always)]
    unsafe fn write(&mut self, value: T) {
        write_volatile(Self::ADDRESS as *mut T, value)
    }
}
