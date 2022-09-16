#![doc(hidden)]
//! Dynamic memory allocation
//!
//! This module provides dynamic memory allocation backed by the BIOS's
//! `malloc`, `init_heap` and `free`.

use crate::sys::kernel;
use core::alloc::{GlobalAlloc, Layout};
use core::cell::Cell;

// SAFETY: We currently ignore threads and interrupts.
unsafe impl Sync for BiosAllocator {}

#[doc(hidden)]
pub struct BiosAllocator {
    /// Number of bytes in use
    pub used: Cell<usize>,
    /// The heap's total size in bytes
    pub total_size: usize,
}

unsafe impl GlobalAlloc for BiosAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // The BIOS malloc is broken and hands out random pointers even when it's
        // out-of-memory. This check doesn't fix all its issues but it does make it
        // easier to detect cases where `no_heap!` can be used allocations don't
        // actually happen even though the alloc crate is used.
        let used = self.used.get();
        if used + layout.size() > self.total_size {
            core::ptr::null_mut()
        } else {
            self.used.set(used + layout.size());
            kernel::psx_malloc(layout.size())
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let used = self.used.get();
        self.used.set(used - layout.size());
        kernel::psx_free(ptr)
    }
}

/// Define a region of memory as a heap managed by the BIOS.
///
/// There may only be one heap per executable and it may be specified in bytes (rounded up to 4), KB or MB. The specified heap will be used by `Box`, `Vector`, `String` and all the other containers in [`alloc`](https://doc.rust-lang.org/alloc/). To use an another allocator implement the [`GlobalAlloc`][core::alloc::GlobalAlloc] trait. Note that the PlayStation BIOS `malloc`s are typically poorly implemented, tend to leak memory and don't OOM correctly. Build with `cargo-psx`'s `--alloc` flag to use this macro. For a reasonably functional alternative see [`heap!`][`crate::heap!`].
///
/// This macro places the heap in the .bss section of the executable so it
/// doesn't take up space, but may slow down executable loaders that make sure
/// to zero out .bss. For more fine-grained control over the heap's placement
/// use [`core::slice::from_raw_parts_mut`] as shown below.
///
/// # Usage
/// ```
/// use psx::sys_heap;
///
/// // sys_heap!(256 bytes);
/// sys_heap!(128 KB);
/// // sys_heap!(1 MB);
///
/// // use psx::constants::*;
/// // sys_heap! {
/// //   SAFETY: This is safe if nothing else has access to the data cache
/// //   unsafe { core::slice::from_raw_parts_mut(DATA_CACHE, DATA_CACHE_LEN)
/// // }
/// ```
#[macro_export]
macro_rules! sys_heap {
    ($n:tt bytes) => {
        $crate::sys_heap! {
            {
                const HEAP_SIZE: usize = ($n + 3) / core::mem::size_of::<u32>();
                static mut HEAP: [u32; HEAP_SIZE] = [0; HEAP_SIZE];
                // SAFETY: This is safe because nothing else in this executable can access
                // `HEAP`
                unsafe { &mut HEAP }
            }
        }
    };
    ($n:tt kb) => { $crate::sys_heap!($n KB); };
    ($n:tt kB) => { $crate::sys_heap!($n KB); };
    ($n:tt KB) => {
        $crate::sys_heap! {
            {
                const HEAP_SIZE: usize = $n * 1024 / core::mem::size_of::<u32>();
                static mut HEAP: [u32; HEAP_SIZE] = [0; HEAP_SIZE];
                // SAFETY: This is safe because nothing else in this executable can access
                // `HEAP`
                unsafe { &mut HEAP }
            }
        }
    };
    ($n:tt Mb) => { $crate::sys_heap!($n MB); };
    ($n:tt MB) => {
        $crate::sys_heap! {
            const HEAP_SIZE: usize = $n * 1024 * 1024 / core::mem::size_of::<u32>();
            static mut HEAP: [u32; HEAP_SIZE] = [0; HEAP_SIZE];
            // SAFETY: This is safe because nothing else in this executable can access
            // `HEAP`
            unsafe { &mut HEAP }
        }
    };
    ($mut_slice:expr) => {
        extern crate alloc;

        #[global_allocator]
        static _HEAP: $crate::sys::heap::BiosAllocator = $crate::sys::heap::BiosAllocator {
            used: core::cell::Cell::new(0),
            total_size: $mut_slice.len() * core::mem::size_of::<u32>(),
        };

        $crate::ctor! {
            fn init_heap() {
                use core::mem::size_of;

                // Type-check the macro argument
                let slice: &'static mut [u32] = $mut_slice;
                let ptr = slice.as_mut_ptr() as usize;
                let len = slice.len() * size_of::<u32>();
                unsafe {
                    $crate::sys::kernel::psx_init_heap(ptr, len);
                }
            }
        }
    };
}
