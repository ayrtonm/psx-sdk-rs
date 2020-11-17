use core::alloc::{GlobalAlloc, Layout};

use crate::bios;
use crate::interrupt;

pub struct BiosAllocator {
    int_mask: interrupt::Mask,
}

// TODO: mutable transmutes are UB, see if there's a way around this without changing interrupt::Mask
// Note `UnsafeCell` and `RefCell` probably won't work because they `impl !Sync` so they can't be in
// statics and global allocators must be static. I might just end up rewriting interrupt::Mask::free
// and directly using volatile_store
#[warn(mutable_transmutes)]
unsafe impl GlobalAlloc for BiosAllocator {
    use core::mem::transmute;
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let int_mask = transmute::<_, &mut interrupt::Mask>(&self.int_mask);
        int_mask.free(|| {
            let ptr = bios::malloc(layout.size());
            // TODO: test `ptr` for errors
            ptr
        })
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let int_mask = transmute::<_, &mut interrupt::Mask>(&self.int_mask);
        int_mask.free(|| {
            bios::free(ptr)
        });
    }
}

#[global_allocator]
pub static HEAP: BiosAllocator = BiosAllocator {
    int_mask: interrupt::Mask,
};

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    loop {}
}
