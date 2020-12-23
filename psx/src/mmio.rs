use core::ptr::{read_volatile, write_volatile};

/// The address of a memory-mapped I/O register.
pub trait Address: Sized {
    /// The address an I/O register is mapped to.
    const ADDRESS: u32;
}

/// An interface for loading an I/O register from memory.
pub trait Load<T>: Address {
    /// Loads an I/O register from memory. Use sparingly as calls cannot be
    /// optimized out by the compiler.
    #[inline(always)]
    unsafe fn load(&self) -> T {
        read_volatile(Self::ADDRESS as *const T)
    }
}

/// An interface for storing an I/O register in memory.
pub trait Store<T>: Address {
    /// Stores an I/O register in memory. Use sparingly as calls cannot be
    /// optimized out by the compiler.
    #[inline(always)]
    unsafe fn store(&mut self, value: T) {
        write_volatile(Self::ADDRESS as *mut T, value)
    }
}
