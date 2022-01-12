use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;
use core::ptr;
use core::ptr::NonNull;

pub struct Global<T>(RefCell<T>);

unsafe impl<T> Sync for Global<T> {}

impl<T> Global<T> {
    pub const fn new(t: T) -> Self {
        Global(RefCell::new(t))
    }

    pub fn interrupt_free<F: FnOnce(&mut T) -> R, R>(&self, f: F) -> R {
        critical_section(|| f(&mut self.0.borrow_mut()))
    }
}

pub fn critical_section<F: FnOnce() -> R, R>(f: F) -> R {
    use crate::hw::cop0::IntSrc;
    use crate::hw::{cop0, Register};
    let mut status = cop0::Status::new();
    if status.interrupt_masked(IntSrc::Hardware) {
        f()
    } else {
        status.mask_interrupt(IntSrc::Hardware).store();
        let res = f();
        status.load().unmask_interrupt(IntSrc::Hardware).store();
        res
    }
}

pub struct Heap(Global<linked_list_allocator::Heap>);

impl Heap {
    pub const fn new() -> Self {
        Self(Global::new(linked_list_allocator::Heap::empty()))
    }

    pub unsafe fn init(&self, base: usize, len: usize) {
        self.0.interrupt_free(|heap| heap.init(base, len))
    }
}

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0
            .interrupt_free(|heap| match heap.allocate_first_fit(layout) {
                Ok(non_null) => non_null.as_ptr(),
                Err(_) => ptr::null_mut(),
            })
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.interrupt_free(|heap| {
            if let Some(ptr) = NonNull::new(ptr) {
                heap.deallocate(ptr, layout);
            }
        })
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
            fn heap() {
                use core::mem::size_of;
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
