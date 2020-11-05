#![feature(lang_items, core_intrinsics)]
#![feature(alloc_error_handler)]
// These are unstable/unfinished features that aren't strictly necessary
#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![feature(array_map)]
#![feature(const_panic)]
#![no_std]
// This library defines the builtin functions, so it would be a shame for
// LLVM to optimize these function calls to themselves!
#![no_builtins]
#![crate_type = "rlib"]
#![crate_name = "libpsx"]

extern crate core;
extern crate alloc;

pub mod bios;
pub mod gpu;
pub mod allocator;
pub mod util;
#[macro_use]
pub(crate) mod constrain;

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if src < dest as *const u8 {
        // copy from end
        let mut i = n;
        while i != 0 {
            i -= 1;
            *dest.offset(i as isize) = *src.offset(i as isize);
        }
    } else {
        // copy from beginning
        let mut i = 0;
        while i < n {
            *dest.offset(i as isize) = *src.offset(i as isize);
            i += 1;
        }
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = c as u8;
        i += 1;
    }
    s
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.offset(i as isize);
        let b = *s2.offset(i as isize);
        if a != b {
            return a as i32 - b as i32;
        }
        i += 1;
    }
    0
}

// Various lang items required by rustc
//#[lang = "stack_exhausted"]
//extern fn stack_exhausted() {}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

//#[lang = "panic_fmt"]
//fn panic_fmt() -> ! { loop {} }
use core::intrinsics;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    intrinsics::abort()
}
