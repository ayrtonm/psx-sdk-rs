use core::cell::UnsafeCell;
use core::hint::unreachable_unchecked;
use core::lazy::Lazy;
use core::ops::{Range, RangeFrom};
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
    #[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
    unsafe fn unwrap_unchecked(self) -> T {
        match self {
            Some(val) => val,
            None => unreachable_unchecked(),
        }
    }
}

/// Returns two mutable slices without runtime checks at the risk of undefined
/// behavior.
#[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
pub const unsafe fn split_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    (
        &mut *slice_from_raw_parts_mut(ptr, mid),
        &mut *slice_from_raw_parts_mut(ptr.add(mid), len - mid),
    )
}

// TODO: Merge all these get_unchecked functions when const fn are allowed in
// traits.
/// Returns a reference to a slice element without runtime checks at the risk of
/// undefined behavior.
#[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
pub const unsafe fn get_unchecked<T>(slice: &[T], idx: usize) -> &T {
    &*slice.as_ptr().add(idx)
}

/// Returns a mutable reference to a slice element without runtime checks at the
/// risk of undefined behavior.
#[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
pub const unsafe fn get_unchecked_mut<T>(slice: &mut [T], idx: usize) -> &mut T {
    &mut *slice.as_mut_ptr().add(idx)
}

/// Returns a reference to a subslice without runtime checks at the risk of
/// undefined behavior.
#[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
pub const unsafe fn get_unchecked_slice<T>(slice: &[T], idx: Range<usize>) -> &[T] {
    let ptr = slice.as_ptr().add(idx.start);
    let len = idx.end - idx.start;
    &*slice_from_raw_parts(ptr, len)
}

/// Returns a mutable reference to a subslice without runtime checks at the risk
/// of undefined behavior.
#[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
pub const unsafe fn get_unchecked_mut_slice<T>(slice: &mut [T], idx: Range<usize>) -> &mut [T] {
    let ptr = slice.as_mut_ptr().add(idx.start);
    let len = idx.end - idx.start;
    &mut *slice_from_raw_parts_mut(ptr, len)
}

/// Returns a reference to a subslice without runtime checks at the risk of
/// undefined behavior.
#[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
pub const unsafe fn get_unchecked_slice_from<T>(slice: &[T], idx: RangeFrom<usize>) -> &[T] {
    let ptr = slice.as_ptr().add(idx.start);
    let len = slice.len() - idx.start;
    &*slice_from_raw_parts(ptr, len)
}

/// Returns a mutable reference to a subslice without runtime checks at the risk
/// of undefined behavior.
#[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
pub const unsafe fn get_unchecked_mut_slice_from<T>(
    slice: &mut [T], idx: RangeFrom<usize>,
) -> &mut [T] {
    let ptr = slice.as_mut_ptr().add(idx.start);
    let len = slice.len() - idx.start;
    &mut *slice_from_raw_parts_mut(ptr, len)
}
