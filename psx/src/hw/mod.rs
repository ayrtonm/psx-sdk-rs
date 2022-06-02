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

pub mod cop0;
pub mod dma;
pub mod gpu;
pub mod gte;
pub mod irq;
pub mod mmio;

use mmio::MemRegister;

/// The four-instruction exception vector.
pub struct ExceptionVector {
    /// The first instruction in the exception vector.
    pub insn_0: MemRegister<u32, 0x8000_0080>,
    /// The second instruction in the exception vector.
    pub insn_1: MemRegister<u32, 0x8000_0084>,
    /// The third instruction in the exception vector.
    pub insn_2: MemRegister<u32, 0x8000_0088>,
    /// The fourth instruction in the exception vector.
    pub insn_3: MemRegister<u32, 0x8000_008C>,
}

impl ExceptionVector {
    const JAL: u32 = 3 << 26;
    const MFC0_R26_EPC: u32 = 0x401a_7000;
    const JR_R26: u32 = 0x0340_0008;
    const RFE: u32 = 0x4200_0010;

    /// Creates a new handle without reading any register values.
    pub fn skip_load() -> Self {
        Self {
            insn_0: MemRegister::skip_load(),
            insn_1: MemRegister::skip_load(),
            insn_2: MemRegister::skip_load(),
            insn_3: MemRegister::skip_load(),
        }
    }

    /// Sets `handler` as the exception handler.
    ///
    /// The set exception handler is responsible for acknowledging pending
    /// interrupts to avoid being called repeatedly. It must also restore any
    /// caller-saved registers it uses to their original values. Failure to
    /// restore any used registers will lead to **undefined behavior**.
    pub fn set_handler(&mut self, handler: extern "C" fn()) {
        fn mask_address(f: extern "C" fn()) -> u32 {
            const MASK: u32 = (1 << 26) - 1;
            let address = (f as u32) >> 2;
            address & MASK
        }
        // Call `handler`
        self.insn_0
            .assign(Self::JAL | mask_address(handler))
            .store();
        // This is in the jump delay slot of JAL so it happens before `handler` is
        // called. This saves the exception program counter EPC.
        self.insn_1.assign(Self::MFC0_R26_EPC).store();

        // This is executed after `handler` returns and jumps back to where the
        // exception happened (EPC).
        self.insn_2.assign(Self::JR_R26).store();
        // This is in the jump delay slot of JR R26 so it happens before returning to
        // EPC.
        self.insn_3.assign(Self::RFE).store();
    }
}

mod private {
    use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};
    pub trait Primitive:
        Copy
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
