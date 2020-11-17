use core::alloc::{GlobalAlloc, Layout};

use crate::bios;

pub struct BiosAllocator;

// This is temporary, see if there's a way to psx::interrupt::Mask::free
// instead w/o relying on UB
mod interrupt {
    pub fn free<F, R>(f: F) -> R
    where F: FnOnce() -> R {
        unsafe {
            core::intrinsics::volatile_store(0x1F80_1074 as *mut u32, 0);
        }
        let ret = f();
        unsafe {
            core::intrinsics::volatile_store(0x1F80_1074 as *mut u32, 0x0000_07FF);
        }
        ret
    }
}

unsafe impl GlobalAlloc for BiosAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        interrupt::free(|| bios::malloc(layout.size()))
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        interrupt::free(|| bios::free(ptr))
    }
}

#[global_allocator]
pub static HEAP: BiosAllocator = BiosAllocator;

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    loop {}
}
