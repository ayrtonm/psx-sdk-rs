use core::alloc::{GlobalAlloc, Layout};

use crate::bios;
use crate::interrupt;

pub struct BiosAllocator;

unsafe impl GlobalAlloc for BiosAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        interrupt::free(|| bios::kernel::malloc(layout.size()))
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        interrupt::free(|| bios::kernel::free(ptr))
    }
}

#[global_allocator]
pub static HEAP: BiosAllocator = BiosAllocator;

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    panic!("Ran out of memory")
}
