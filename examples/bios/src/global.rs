use core::cell::UnsafeCell;
use psx::CriticalSection;

#[repr(transparent)]
pub struct Global<T>(UnsafeCell<T>);

unsafe impl<T> Sync for Global<T> {}

impl<T> Global<T> {
    /// Creates a new Global
    pub const fn new(t: T) -> Self {
        Self(UnsafeCell::new(t))
    }

    /// Gets a pointer to the Global
    ///
    /// Note that other threads may also have pointers to the Global.
    pub const fn as_ptr(&self) -> *mut T {
        self.0.get()
    }

    /// Mutably borrows the Global by ensuring we're running in a critical
    /// section
    pub fn borrow(&self, _: &mut CriticalSection) -> &mut T {
        let ptr = self.as_ptr();
        let opt_ref = unsafe { ptr.as_mut() };
        opt_ref.unwrap()
    }
}
