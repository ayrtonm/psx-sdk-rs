#![feature(lang_items,asm,core_intrinsics)]
#![no_std]
// This library defines the builtin functions, so it would be a shame for
// LLVM to optimize these function calls to themselves!
#![no_builtins]
#![crate_type = "rlib"]
#![crate_name = "libpsx"]

extern crate core;

pub mod bios;

#[no_mangle]
pub extern fn memset(dst: *mut u8, b: i32, len: u32) -> *mut u8 {
    for i in 0..len {
        unsafe {
            *dst.offset(i as isize) = b as u8;
        }
    }

    dst
}

#[no_mangle]
pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8,
                            n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    return dest;
}

// Various lang items required by rustc
//#[lang = "stack_exhausted"]
//extern fn stack_exhausted() {}

#[lang = "eh_personality"]
extern fn eh_personality() {}

//#[lang = "panic_fmt"]
//fn panic_fmt() -> ! { loop {} }
use core::intrinsics;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    intrinsics::abort()
}
