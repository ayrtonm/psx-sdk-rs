use core::alloc::{GlobalAlloc, Layout};
use core::ptr;

#[cfg(feature = "heap")]
pub mod linked_list;

pub struct NoHeap;

unsafe impl GlobalAlloc for NoHeap {
    unsafe fn alloc(&self, _: Layout) -> *mut u8 {
        ptr::null_mut()
    }
    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {}
}

/// Defines a global allocator without a backing heap.
///
/// This macros lets you use the types and methods in the `alloc` crate without
/// a backing heap. This is useful for methods that do not allocate, such as
/// sorting. Any attempts to allocate will instantly give an OOM error. This is
/// a dependency-free alternative to heap!(0 bytes).
#[macro_export]
macro_rules! no_heap {
    () => {
        extern crate alloc;

        #[global_allocator]
        static _HEAP: $crate::heap::NoHeap = $crate::heap::NoHeap;
    };
}
