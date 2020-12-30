use core::cell::UnsafeCell;
use core::lazy::Lazy;

/// A global variable
pub struct Global<T>(Lazy<UnsafeCell<T>>);

unsafe impl<T> Sync for Global<T> {}

impl<T> Global<T> {
    /// Creates a new `Global` in a const context.
    pub const unsafe fn new(f: fn() -> UnsafeCell<T>) -> Self {
        Global(Lazy::new(f))
    }

    /// Gets an immutable reference to the global.
    pub fn get(&self) -> &T {
        unsafe { &*self.0.get() }
    }

    /// Gets a mutable reference to the global.
    pub fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.0.get() }
    }
}
