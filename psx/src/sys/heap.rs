#![doc(hidden)]
//! Dynamic memory allocation
//!
//! This module provides dynamic memory allocation backed by the BIOS's
//! `malloc`, `init_heap` and `free`.

use crate::sys::kernel;
use core::alloc::{GlobalAlloc, Layout};

#[doc(hidden)]
pub struct BiosAllocator;

unsafe impl GlobalAlloc for BiosAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        kernel::malloc(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        kernel::free(ptr)
    }
}

/// Define a region of memory specified by a mutable slice as a heap managed by
/// the BIOS.
///
/// Note that the PlayStation BIOS `malloc`s are typically poorly implemented
/// and tend to leak memory. For a reasonable alternative see [`heap`].
/// # Usage
/// ```
/// use core::slice;
/// use psx::sys_heap;
/// use psx::constants::*;
///
/// // SAFETY: This is safe since we are not using the data cache for anything else.
/// sys_heap!(unsafe {
///     slice::from_raw_parts_mut(DATA_CACHE, DATA_CACHE_LEN)
/// })
/// ```
#[macro_export]
macro_rules! sys_heap {
    ($mut_slice:expr) => {
        extern crate alloc;

        #[global_allocator]
        static _HEAP: $crate::sys::heap::BiosAllocator = $crate::sys::heap::BiosAllocator;

        $crate::ctor! {
            fn init_heap() {
                use core::mem::size_of;

                // Type-check the macro argument
                let slice: &'static mut [u32] = $mut_slice;
                let ptr = slice.as_mut_ptr() as usize;
                let len = slice.len() * size_of::<u32>();
                unsafe {
                    kernel::init_heap(ptr, len);
                }
            }
        }
    };
}
