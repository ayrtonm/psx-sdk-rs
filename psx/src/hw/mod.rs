//! Direct hardware access.
//!
//! This module provides direct access to memory-mapped I/O and coprocessor
//! registers. Structs implementing the `Register` trait represent ownership of
//! each hardware register. Each struct contains a non-volatile copy of the
//! register's value which the struct's methods operate on. This value must then
//! be stored to write it to the hardware register.

#![allow(non_camel_case_types)]

// This module has a macro for defining coprocessor registers.
#[macro_use]
pub mod cop;

pub mod cdrom;
pub mod cop0;
pub mod dma;
pub mod gpu;
pub mod gte;
pub mod irq;
pub mod mmio;

use mmio::MemRegister;

mod private {
    use core::fmt::Debug;
    use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};
    pub trait Primitive:
        Copy
        + Debug
        + PartialEq
        + Not<Output = Self>
        + BitAnd<Output = Self>
        + BitOr<Output = Self>
        + BitXor<Output = Self>
        + BitAndAssign
        + BitOrAssign
        + BitXorAssign
        + From<u8> {
    }

    impl Primitive for u8 {}
    impl Primitive for u16 {}
    impl Primitive for u32 {}
    impl Primitive for i16 {}
    impl Primitive for i32 {}
}

/// A coprocessor or memory-mapped I/O register.
///
/// Implementors keep a copy of the register's value cached to allow making
/// multiple logical operations between each volatile access. It is the user's
/// responsibility to keep the cached value in sync with the register if
/// necessary.
pub trait Register<T: private::Primitive>: Sized + AsRef<T> + AsMut<T> {
    /// Creates a new handle without reading the register's value.
    ///
    /// This should not do any volatile reads.
    fn skip_load() -> Self;

    /// Creates a new handle and immediately reads the register's value.
    ///
    /// This does a single volatile read.
    fn new() -> Self {
        let mut reg = Self::skip_load();
        reg.load();
        reg
    }

    /// Load the register's value into a cache.
    ///
    /// This does a single volatile read.
    fn load(&mut self) -> &mut Self;

    /// Store the cached value in the register.
    ///
    /// This does a single volatile write.
    fn store(&mut self) -> &mut Self;

    /// Creates a new handle with the cached value set to the specified value.
    fn from_bits(bits: T) -> Self {
        let mut reg = Self::skip_load();
        reg.assign(bits);
        reg
    }
    /// Gets the cached value.
    fn to_bits(&self) -> T {
        *self.as_ref()
    }

    /// Sets the cached value to `bits`.
    fn assign(&mut self, bits: T) -> &mut Self {
        *self.as_mut() = bits;
        self
    }

    /// Checks if any specified bits are set in the cached value.
    fn any_set(&self, bits: T) -> bool {
        *self.as_ref() & bits != T::from(0)
    }

    /// Checks if all specified bits are set in the cached value.
    fn all_set(&self, bits: T) -> bool {
        *self.as_ref() & bits == bits
    }

    /// Checks if all specified bits are cleared in the cached value.
    fn all_clear(&self, bits: T) -> bool {
        *self.as_ref() & bits == T::from(0)
    }

    /// Sets the specified bits in the cached value.
    fn set_bits(&mut self, bits: T) -> &mut Self {
        *self.as_mut() |= bits;
        self
    }

    /// Clears the specified bits in the cached value.
    fn clear_bits(&mut self, bits: T) -> &mut Self {
        *self.as_mut() &= !bits;
        self
    }

    /// Toggles the specified bits in the cached value.
    fn toggle_bits(&mut self, bits: T) -> &mut Self {
        *self.as_mut() ^= bits;
        self
    }

    /// Sets all bits in the cached value.
    fn set_all(&mut self) -> &mut Self {
        *self.as_mut() = !T::from(0);
        self
    }

    /// Clears all bits in the cached value.
    fn clear_all(&mut self) -> &mut Self {
        *self.as_mut() = T::from(0);
        self
    }

    /// Toggles all bits in the cached value.
    fn toggle_all(&mut self) -> &mut Self {
        *self.as_mut() ^= !T::from(0);
        self
    }
}
