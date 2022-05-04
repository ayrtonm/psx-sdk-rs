use core::alloc::{GlobalAlloc, Layout};
use core::borrow::BorrowMut;
use core::cell::RefCell;
use core::ptr;
use core::ptr::NonNull;

pub struct Heap(RefCell<linked_list_allocator::Heap>);

// SAFETY: We currently ignore threads and interrupts.
unsafe impl Sync for Heap {}

impl Heap {
    pub const fn new() -> Self {
        Self(RefCell::new(linked_list_allocator::Heap::empty()))
    }

    pub unsafe fn init(&self, base: usize, len: usize) {
        self.0.borrow_mut().borrow_mut().init(base, len)
    }
}

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match self.0.borrow_mut().allocate_first_fit(layout) {
            Ok(non_null) => non_null.as_ptr(),
            Err(_) => ptr::null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if let Some(non_null) = NonNull::new(ptr) {
            self.0.borrow_mut().deallocate(non_null, layout);
        };
    }
}

/// Define a region of memory specified by a mutable slice as a heap.
#[macro_export]
macro_rules! heap {
    ($mut_slice:expr) => {
        extern crate alloc;

        #[global_allocator]
        static _HEAP: $crate::heap::Heap = $crate::heap::Heap::new();

        $crate::ctor! {
            fn init_heap() {
                use core::mem::size_of;

                // Type-check the macro argument
                let slice: &'static mut [u32] = $mut_slice;
                let ptr = slice.as_mut_ptr() as usize;
                let len = slice.len() * size_of::<u32>();
                unsafe {
                    _HEAP.init(ptr, len);
                }
            }
        }
    };
}
