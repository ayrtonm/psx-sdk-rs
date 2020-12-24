use core::ptr::{read_volatile, write_volatile};

use crate::value;
use crate::value::{Load, LoadMut};

/// An addressable memory-mapped I/O register.
pub trait Address<T> {
    /// The memory address the register is mapped to.
    const ADDRESS: u32;
}

/// A marker for readable memory-mapped I/O registers.
pub trait Read<T>: Address<T> {}
/// A marker for writeable memory-mapped I/O registers.
pub trait Write<T>: Address<T> {}

// Implementing `Load<T>` for a memory-mapped I/O register automatically
// provides an implementation of `value::Read`.
impl<T: Copy, R: Load<T> + Address<T>> Read<T> for R {}
impl<T: Copy, R: Read<T>> value::Read<T> for R {
    /// Loads an I/O register from memory. Use sparingly as calls cannot be
    /// optimized out by the compiler.
    #[inline(always)]
    unsafe fn read(&self) -> T {
        read_volatile(Self::ADDRESS as *const T)
    }
}

// Implementing `LoadMut<T>` for a memory-mapped I/O register automatically
// provides an implementation of `value::Write`.
impl<T: Copy + Default, R: LoadMut<T> + Address<T>> Write<T> for R {}
impl<T: Copy + Default, R: Write<T>> value::Write<T> for R {
    /// Stores an I/O register in memory. Use sparingly as calls cannot be
    /// optimized out by the compiler.
    #[inline(always)]
    unsafe fn write(&mut self, value: T) {
        write_volatile(Self::ADDRESS as *mut T, value)
    }
}
