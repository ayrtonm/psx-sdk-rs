use core::cell::UnsafeCell;
use core::lazy::Lazy;

/// A global variable with lazy initialization.
pub struct LazyGlobal<T>(Lazy<UnsafeCell<T>>);

unsafe impl<T> Sync for LazyGlobal<T> {}

/// Creates a lazy global binding.
#[macro_export]
macro_rules! global {
    (let $binding:ident : $type:ty = $value:expr) => {
        pub static $binding: $crate::lazy_global::LazyGlobal<$type> =
            $crate::lazy_global::LazyGlobal::new(|| core::cell::UnsafeCell::new(unsafe { $value }));
    };
}

impl<T> LazyGlobal<T> {
    /// Creates a new `LazyGlobal` in a const context.
    pub const fn new(f: fn() -> UnsafeCell<T>) -> Self {
        LazyGlobal(Lazy::new(f))
    }

    /// Gets a mutable reference to the global.
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    pub fn get(&self) -> &mut T {
        unsafe { &mut *self.0.get() }
    }
}
