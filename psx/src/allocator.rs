use core::alloc::{GlobalAlloc, Layout};

use crate::bios;
use crate::interrupt;

pub struct BiosAllocator;

unsafe impl GlobalAlloc for BiosAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // TODO: Verify that this unsafe fn is ok here
        let mut int_mask = interrupt::Mask::new();
        int_mask.free(|| bios::malloc(layout.size()))
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        // TODO: Verify that this unsafe fn is ok here
        let mut int_mask = interrupt::Mask::new();
        int_mask.free(|| bios::free(ptr))
    }
}

#[global_allocator]
pub static HEAP: BiosAllocator = BiosAllocator;

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    loop {}
}
