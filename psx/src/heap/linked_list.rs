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

    pub unsafe fn init(&self, base: *mut u8, len: usize) {
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

/// Defines a region of memory as a heap managed by [`linked_list_allocator`](https://crates.io/crates/linked_list_allocator).
///
/// There may only be one heap per executable and it may be specified in bytes (rounded up to 4), KB or MB. The specified heap will be used by `Box`, `Vector`, `String` and all the other containers in [`alloc`](https://doc.rust-lang.org/alloc/). To use an another allocator implement the [`GlobalAlloc`][core::alloc::GlobalAlloc] trait. Enable this crate's [`heap` feature][`crate`] and build with `cargo-psx`'s `--alloc` flag to use this macro. For a dependency-free allocator see [`sys_heap!`][`crate::sys_heap!`].
///
/// Note that this macro places the heap in the .bss section of the executable
/// so it doesn't take up space, but may slow down executable loaders that make
/// sure to zero out .bss. For more fine-grained control over the heap's
/// placement use [`core::slice::from_raw_parts_mut`] as shown below.
///
/// # Usage
/// ```
/// use psx::heap;
///
/// // heap!(256 bytes);
/// heap!(128 KB);
/// // heap!(1 MB);
///
/// // use core::slice;
/// // use core::mem::size_of;
/// // use psx::constants::*;
/// // heap! {
/// //   SAFETY: This is safe if nothing else has access to the data cache
/// //   let ptr = (KSEG0 + DATA_CACHE) as *mut u32;
/// //   let len = DATA_CACHE_LEN / size_of::<u32>();
/// //   unsafe { slice::from_raw_parts_mut(ptr, len)
/// // }
/// ```
#[macro_export]
macro_rules! heap {
    ($n:tt bytes) => {
        $crate::heap! {
            {
                const HEAP_SIZE: usize = ($n + 3) / core::mem::size_of::<u32>();
                static mut HEAP: [u32; HEAP_SIZE] = [0; HEAP_SIZE];
                // SAFETY: This is safe because nothing else in this executable can access
                // `HEAP`
                unsafe { &mut HEAP }
            }
        }
    };
    ($n:tt kb) => { $crate::heap!($n KB); };
    ($n:tt kB) => { $crate::heap!($n KB); };
    ($n:tt KB) => {
        $crate::heap! {
            {
                const HEAP_SIZE: usize = $n * 1024 / core::mem::size_of::<u32>();
                static mut HEAP: [u32; HEAP_SIZE] = [0; HEAP_SIZE];
                // SAFETY: This is safe because nothing else in this executable can access
                // `HEAP`
                unsafe { &mut HEAP }
            }
        }
    };
    ($n:tt Mb) => { $crate::heap!($n MB); };
    ($n:tt MB) => {
        $crate::heap! {
            {
                const HEAP_SIZE: usize = $n * 1024 * 1024 / core::mem::size_of::<u32>();
                static mut HEAP: [u32; HEAP_SIZE] = [0; HEAP_SIZE];
                // SAFETY: This is safe because nothing else in this executable can access
                // `HEAP`
                unsafe { &mut HEAP }
            }
        }
    };
    ($mut_slice:expr) => {
        extern crate alloc;

        #[global_allocator]
        static _HEAP: $crate::heap::linked_list::Heap = $crate::heap::linked_list::Heap::new();

        $crate::ctor! {
            fn init_heap() {
                use core::mem::size_of;

                // Type-check the macro argument
                let slice: &'static mut [u32] = $mut_slice;
                let ptr = slice.as_mut_ptr().cast();
                let len = slice.len() * size_of::<u32>();
                unsafe {
                    _HEAP.init(ptr, len);
                }
            }
        }
    };
}
