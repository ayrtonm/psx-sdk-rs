use core::alloc::{GlobalAlloc, Layout};

use crate::bios;

pub struct BiosAllocator;

impl BiosAllocator {
    pub fn init(addr: usize, size: usize) {
        bios::init_heap(addr, size);
    }
}

// TODO: Make this impl interrupt-free as seen here
// https://docs.rust-embedded.org/book/collections/
// This'll probably require some small bits of assembly and glue to modify interrupts
unsafe impl GlobalAlloc for BiosAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = bios::malloc(layout.size());
        // TODO: test for errors
        //use crate::gpu::position::*;
        //use crate::gpu::color::*;
        //use crate::gpu::polygon::*;
        //if ptr == core::ptr::null_mut() {
        //    draw_square(&Position::zero(), 16, &Color::red(), &Opacity::Opaque);
        //} else {
        //    let p = Position::new(16,0);
        //    draw_square(&p, 16, &Color::green(), &Opacity::Opaque);
        //};
        ptr
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        bios::free(ptr)
    }
}

#[global_allocator]
pub static HEAP: BiosAllocator = BiosAllocator;

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! { loop { } }
