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

#[cfg(test)]
mod tests {
    #[test_case]
    fn malloc_() {
        use crate::bios;
        let mut mem = [0; 256];
        bios::init_heap(&mut mem);
        let alloc = bios::malloc(16);
        alloc[0] = 0xff;
        assert!(alloc.len() == 16);
        assert!(alloc[0] == 0xff);
        for i in 1..16 {
            assert!(alloc[i] == 0);
        }
        bios::free(alloc);
        unsafe {
            bios::kernel::init_heap(0, 0);
        }
    }
}
