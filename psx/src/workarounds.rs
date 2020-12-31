use core::cell::UnsafeCell;
use core::hint::unreachable_unchecked;
use core::lazy::Lazy;

/// A global variable with lazy initialization.
pub struct LazyGlobal<T>(Lazy<UnsafeCell<T>>);

unsafe impl<T> Sync for LazyGlobal<T> {}

/// Creates a lazy global binding.
#[macro_export]
macro_rules! lazy_global {
    (let $binding:ident : $type:ty = $value:expr) => {
        pub static $binding: $crate::workarounds::LazyGlobal<$type> =
            $crate::workarounds::LazyGlobal::new(|| core::cell::UnsafeCell::new(unsafe { $value }));
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

/// Removes runtime panic-checks when unwrapping options at the risk of
/// undefined behavior.
pub trait UnwrapUnchecked<T> {
    /// Returns a result without runtime checks.
    fn unwrap_unchecked(self) -> T;
}

impl<T> UnwrapUnchecked<T> for Option<T> {
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    fn unwrap_unchecked(self) -> T {
        match self {
            Some(val) => val,
            None => unsafe { unreachable_unchecked() },
        }
    }
}

/// Removes runtime panic-checks when splitting slices at the risk of undefined
/// behavior.
pub trait SplitAtMutNoCheck<T> {
    /// Returns two mutable slices without runtime checks.
    fn split_at_mut_no_check(&mut self, mid: usize) -> (&mut [T], &mut [T]);
}

impl<T> SplitAtMutNoCheck<T> for [T] {
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    fn split_at_mut_no_check(&mut self, mid: usize) -> (&mut [T], &mut [T]) {
        use core::slice;

        let len = self.len();
        let ptr = self.as_mut_ptr();
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
}
