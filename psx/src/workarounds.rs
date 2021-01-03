use core::cell::UnsafeCell;
use core::lazy::Lazy;
use core::ops::{Range, RangeFrom};

#[cfg(not(feature = "forbid_UB"))]
use core::hint::unreachable_unchecked;
#[cfg(not(feature = "forbid_UB"))]
use core::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};

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
    unsafe fn unwrap_unchecked(self) -> T;
}

impl<T> UnwrapUnchecked<T> for Option<T> {
    #[cfg(feature = "forbid_UB")]
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    unsafe fn unwrap_unchecked(self) -> T {
        self.unwrap()
    }

    #[cfg(not(feature = "forbid_UB"))]
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    unsafe fn unwrap_unchecked(self) -> T {
        match self {
            Some(val) => val,
            None => unreachable_unchecked(),
        }
    }
}

// The UB version of these functions can be made const since they are used for
// const testing. However, I don't want the `forbid_UB` feature to affect the
// const-ness of any functions so outside testing they'll always be non-const
macro_rules! duplicate_without_ub {
    ($alt_body:expr, $(#[$($meta:meta)*])* pub unsafe fn $ident:ident <T> ($($args:tt)*) -> $ret:ty { $($body:tt)* }) => {
        // If UB is allowed, use the specified function body
        #[cfg(not(feature = "forbid_UB"))]
        $crate::const_for_tests! {
            $(#[$($meta)*])* pub unsafe fn $ident <T> ($($args)*) -> $ret { $($body)* }
        }
        // If UB is forbidden, use the alterante expression as the function body
        #[cfg(feature = "forbid_UB")]
        $(#[$($meta)*])* pub unsafe fn $ident <T> ($($args)*) -> $ret { $alt_body }
    };
}

duplicate_without_ub! {
    slice.split_at_mut(mid),
    /// Returns two mutable slices without runtime checks at the risk of undefined
    /// behavior.
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    pub unsafe fn split_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();
        (
            &mut *slice_from_raw_parts_mut(ptr, mid),
            &mut *slice_from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// TODO: Merge all these get_unchecked functions when const fn are allowed in
// traits.
duplicate_without_ub! {
    &slice[idx],
    /// Returns a reference to a slice element without runtime checks at the risk of
    /// undefined behavior.
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    pub unsafe fn get_unchecked<T>(slice: &[T], idx: usize) -> &T {
        &*slice.as_ptr().add(idx)
    }
}

duplicate_without_ub! {
    &mut slice[idx],
    /// Returns a mutable reference to a slice element without runtime checks at the
    /// risk of undefined behavior.
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    pub unsafe fn get_unchecked_mut<T>(slice: &mut [T], idx: usize) -> &mut T {
        &mut *slice.as_mut_ptr().add(idx)
    }
}

duplicate_without_ub! {
    &slice[idx],
    /// Returns a reference to a subslice without runtime checks at the risk of
    /// undefined behavior.
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    pub unsafe fn get_unchecked_slice<T>(slice: &[T], idx: Range<usize>) -> &[T] {
        let ptr = slice.as_ptr().add(idx.start);
        let len = idx.end - idx.start;
        &*slice_from_raw_parts(ptr, len)
    }
}

duplicate_without_ub! {
    &mut slice[idx],
    /// Returns a mutable reference to a subslice without runtime checks at the risk
    /// of undefined behavior.
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    pub unsafe fn get_unchecked_mut_slice<T>(slice: &mut [T], idx: Range<usize>) -> &mut [T] {
        let ptr = slice.as_mut_ptr().add(idx.start);
        let len = idx.end - idx.start;
        &mut *slice_from_raw_parts_mut(ptr, len)
    }
}

duplicate_without_ub! {
    &slice[idx],
    /// Returns a reference to a subslice without runtime checks at the risk of
    /// undefined behavior.
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    pub unsafe fn get_unchecked_slice_from<T>(slice: &[T], idx: RangeFrom<usize>) -> &[T] {
        let ptr = slice.as_ptr().add(idx.start);
        let len = slice.len() - idx.start;
        &*slice_from_raw_parts(ptr, len)
    }
}

duplicate_without_ub! {
    &mut slice[idx],
    /// Returns a mutable reference to a subslice without runtime checks at the risk
    /// of undefined behavior.
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    pub unsafe fn get_unchecked_mut_slice_from<T>(
        slice: &mut [T], idx: RangeFrom<usize>,
    ) -> &mut [T] {
        let ptr = slice.as_mut_ptr().add(idx.start);
        let len = slice.len() - idx.start;
        &mut *slice_from_raw_parts_mut(ptr, len)
    }
}
