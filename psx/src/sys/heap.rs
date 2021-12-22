#![doc(hidden)]
//! Dynamic memory allocation
//!
//! This module provides dynamic memory allocation backed by the BIOS's
//! `malloc`, `init_heap` and `free`.

use crate::sys::{critical_section, kernel};
use core::alloc::{GlobalAlloc, Layout};
use core::ops::{Deref, DerefMut};
use core::slice;

#[doc(hidden)]
pub struct BiosAllocator;

unsafe impl GlobalAlloc for BiosAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        super::critical_section(|| kernel::malloc(layout.size()))
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        super::critical_section(|| kernel::free(ptr))
    }
}

/// Define a region of memory specified by a mutable slice as a BIOS-backed heap.
#[macro_export]
macro_rules! heap {
    ($mut_slice:expr) => {
        extern crate alloc;

        #[global_allocator]
        static _HEAP: psx::sys::heap::BiosAllocator = psx::sys::heap::BiosAllocator;

        #[used]
        #[link_section = ".ctors"]
        static _INIT_HEAP: fn() = _init_heap;

        fn _init_heap() {
            use core::mem::size_of;
            use psx::sys::kernel;

            // Type-check the macro argument
            let slice: &'static mut [u32] = $mut_slice;
            let ptr = slice.as_mut_ptr() as usize;
            let len = slice.len() * size_of::<usize>();
            unsafe {
                kernel::init_heap(ptr, len);
            }
        }
    };
}

#[cfg(not(feature = "custom_oom"))]
#[alloc_error_handler]
fn on_oom(layout: core::alloc::Layout) -> ! {
    panic!("Ran out of memory {:?}", layout);
}
