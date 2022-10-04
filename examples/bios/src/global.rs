use core::cell::UnsafeCell;
use psx::hw::cop0::IntSrc;
use psx::hw::{cop0, Register};

pub struct Global<T>(UnsafeCell<T>);

unsafe impl<T> Sync for Global<T> {}

impl<T> Global<T> {
    pub const fn new(t: T) -> Self {
        Self(UnsafeCell::new(t))
    }

    /// Assumes that we can mutably access the Global and returns a mutable
    /// reference to it.
    ///
    /// # SAFETY: No other reference to the Global may be created during the
    /// lifetime of the return value.
    pub unsafe fn assume_mut(&self) -> &mut T {
        unsafe { self.0.get().as_mut().unwrap() }
    }

    /// Ensures that we can mutably access the Global then calls a closure with
    /// a mutable reference to it and returns the result.
    ///
    /// Note that any modifications to cop0r12 in the closure must be done
    /// through the closure's second argument.
    pub fn ensure_mut<F: Fn(&mut T, &mut cop0::Status) -> R, R>(
        &self, sr: &mut cop0::Status, f: F,
    ) -> R {
        // Check if we are already in a critical section
        let in_critical_section = sr.interrupt_masked(IntSrc::Hardware);
        // If we are not in a critical section, mask the hardware interrupt
        if !in_critical_section {
            sr.mask_interrupt(IntSrc::Hardware).store();
        }
        // SAFETY: This is safe to access since we are now in a critical section
        let global = unsafe { self.assume_mut() };
        let res = f(global, sr);

        if !in_critical_section {
            // If we were not in a critical section, unmask the hardware
            // interrupt again
            sr.unmask_interrupt(IntSrc::Hardware).store();
        }
        res
    }
}
