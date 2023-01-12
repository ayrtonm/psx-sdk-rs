use core::cell::UnsafeCell;

#[repr(transparent)]
pub struct Global<T>(UnsafeCell<T>);

unsafe impl<T> Sync for Global<T> {}

impl<T> Global<T> {
    pub const fn new(t: T) -> Self {
        Self(UnsafeCell::new(t))
    }

    pub const fn as_ptr(&self) -> *mut T {
        self.0.get()
    }

    /// Gets a mutable reference to the Global
    ///
    /// # SAFETY: No other reference to the Global may exist during the lifetime of the return value
    pub const unsafe fn as_ref(&self) -> &mut T {
        let ptr = self.as_ptr();
        let opt_ref = ptr.as_mut();
        opt_ref.unwrap()
    }
}
