//! Hardware abstraction layer for memory-mapped I/O and coprocessor registers.
//!
//! This module provides an API for using memory-mapped I/O and coprocessor
//! registers directly. Volatile accesses are not elided by the compiler, so to
//! avoid unnecessary reads/writes readable register handles store a copy of
//! their value. For read-write registers this value must be explicitly written
//! back to the register.
//!
//! ```rust
//! // `Shared` registers can be `mut` to allow updating their stored value.
//! let mut irq_stat = I_STAT::<Shared>::load();
//! // waits for the next Vblank interrupt request then updates the stored value
//! // when the register is changed by the hardware.
//! irq_stat.wait(IRQ::Vblank);
//! // Acknowledging interrupt requests is not allowed here since writing to a
//! // `Shared` register is not allowed.
//! // irq_stat.ack(IRQ::CDROM).store();
//!
//! // Create a `Mutable` handle to use methods that write to registers.
//! let mut irq_mask = I_MASK::<Mutable>::load();
//! // The `enable_irq` method is available since the handle is `Mutable`.
//! irq_mask.enable_irq(IRQ::Timer0);
//! // The new value must then be stored in the register.
//! irq_mask.store();
//!
//! // If a write will modify the entire register, use `skip_load` to
//! // avoid reading the register.
//! let mut dma_control = DPCR::skip_load();
//! // `dma_control`'s stored value probably differs the register's current
//! // value, but `enable_all` will change all of the relevant bits. Also since
//! // `enable_all` returns `&mut Self`, `store` can be chained on.
//! dma_control.enable_all().store();
//! ```

#![allow(non_camel_case_types)]

#[macro_use]
mod macros;

mod mmio;
pub use mmio::{D0_BCR, D0_CHCR, D0_MADR};
pub use mmio::{D1_BCR, D1_CHCR, D1_MADR};
pub use mmio::{D2_BCR, D2_CHCR, D2_MADR};
pub use mmio::{D3_BCR, D3_CHCR, D3_MADR};
pub use mmio::{D4_BCR, D4_CHCR, D4_MADR};
pub use mmio::{D5_BCR, D5_CHCR, D5_MADR};
pub use mmio::{D6_BCR, D6_CHCR, D6_MADR};
pub use mmio::{DICR, DPCR};
pub use mmio::{GP0, GP1, GPUREAD, GPUSTAT};
pub use mmio::{I_MASK, I_STAT};
pub use mmio::{T0_CNT, T0_MODE, T0_TGT};
pub use mmio::{T1_CNT, T1_MODE, T1_TGT};
pub use mmio::{T2_CNT, T2_MODE, T2_TGT};

#[macro_use]
mod asm;

pub mod cop0;
pub mod cpu;
pub mod gte;

/// Direct memory access channel and control registers.
pub mod dma;
/// GPU registers.
pub mod gpu;
/// Interrupt request registers.
pub mod irq;
/// Timer registers.
pub mod timer;

/// The address corresponding to a
/// [memory-mapped I/O register](http://problemkaputt.de/psx-spx.htm#iomap).
pub trait Address {
    /// The 32-bit address corresponding to the register. Note there is no
    /// alignment constraint.
    const ADDRESS: u32;
}

/// Volatile memory-mapped I/O or coprocessor register reads.
pub trait Read<T> {
    /// Returns the register's current value. Note that the read will not be
    // /elided by the compiler.
    fn read(&self) -> T;
}

/// Volatile memory-mapped I/O or coprocessor register writes.
///
/// Note that the write(s) will not be elided by the compiler.
pub trait Write<T: Copy> {
    /// Writes `value` to the register.
    fn write(&mut self, value: T);

    /// Writes the slice of `values` to the register.
    fn write_slice(&mut self, values: &[T]) {
        for &v in values {
            self.write(v);
        }
    }
}

// Seal the `Register` and `MutRegister` traits.
mod private {
    use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

    pub trait HasValue<T> {
        fn get(&self) -> T;
        fn get_mut(&mut self) -> &mut T;
    }

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
}

/// Basic operations for all register handles.
///
/// All methods may be elided by the compiler unless stated otherwise.
pub trait Register<T: private::Primitive>: private::HasValue<T> + Read<T> {
    /// Reads the register and creates a handle with a copy of the register's
    /// current value. Note that the read will not be elided by the compiler.
    fn load() -> Self;

    /// Returns the handle's copy of the register.
    fn bits(&self) -> T {
        self.get()
    }

    /// Updates the handle's copy of the register. Note that the read will not
    /// be elided by the compiler. Returns `&mut Self` for convenience.
    fn reload(&mut self) -> &mut Self {
        let new_value = self.read();
        *self.get_mut() = new_value;
        self
    }

    /// Checks if any of the given `bits` are set in the handle's copy of the
    /// register.
    fn any_set(&self, bits: T) -> bool {
        self.get() & bits != T::from(0)
    }

    /// Checks if all of the given `bits` are set in the handle's copy of the
    /// register.
    fn all_set(&self, bits: T) -> bool {
        self.get() & bits == bits
    }

    /// Checks if all of the given `bits` are cleared in the handle's copy
    /// of the register.
    fn all_cleared(&self, bits: T) -> bool {
        self.get() & bits == T::from(0)
    }
}

/// Basic operations for [`Mutable`] register handles.
///
/// All methods may be elided by the compiler unless stated otherwise. Most
/// methods also return `&mut Self` for convenience.
pub trait MutRegister<T: private::Primitive>: Sized + Register<T> + Write<T> {
    /// Creates a handle without reading the register's current value.
    fn skip_load() -> Self;

    /// Writes the handle's copy of the register's value to the register. Note
    /// that the write will not be elided by the compiler.
    fn store(&mut self) -> &mut Self {
        self.write(self.get());
        self
    }

    /// Sets the handle's copy of the register to `bits`.
    fn assign(&mut self, bits: T) -> &mut Self {
        *self.get_mut() = bits;
        self
    }

    /// Sets the given `bits` in the handle's copy of the register.
    fn set_bits(&mut self, bits: T) -> &mut Self {
        *self.get_mut() |= bits;
        self
    }

    /// Clears the given `bits` in the handle's copy of the register.
    fn clear_bits(&mut self, bits: T) -> &mut Self {
        *self.get_mut() &= !bits;
        self
    }

    /// Toggles the given `bits` in the handle's copy of the register.
    fn toggle_bits(&mut self, bits: T) -> &mut Self {
        *self.get_mut() ^= bits;
        self
    }

    /// Sets all bits in the handle's copy of the register.
    fn set_all(&mut self) -> &mut Self {
        *self.get_mut() |= !T::from(0);
        self
    }

    /// Clears all bits in the handle's copy of the register.
    fn clear_all(&mut self) -> &mut Self {
        *self.get_mut() &= T::from(0);
        self
    }

    /// Toggles all bits in the handle's copy of the register.
    fn toggle_all(&mut self) -> &mut Self {
        *self.get_mut() ^= !T::from(0);
        self
    }
}

/// A marker type for a register handle which may be shared between threads.
pub struct Shared {}
/// A marker type for a mutable register handle.
pub struct Mutable {}
/// A marker trait denoting the mutability of a register handle.
pub trait State {}
impl State for Shared {}
impl State for Mutable {}
