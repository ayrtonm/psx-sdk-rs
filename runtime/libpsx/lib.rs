#![feature(no_std,core,lang_items,asm)]
#![no_std]
#![crate_type = "rlib"]
#![crate_name = "psx"]

extern crate core;

pub mod uart;

#[no_mangle]
pub extern fn memset(dst: *mut u8, b: i32, len: u32) -> *mut u8 {
    for i in 0..len {
        unsafe {
            *dst.offset(i as isize) = b as u8;
        }
    }

    dst
}

// Various lang items required by rustc
#[lang = "stack_exhausted"]
extern fn stack_exhausted() {}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
fn panic_fmt() -> ! { loop {} }
