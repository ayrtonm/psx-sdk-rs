#![feature(no_std,core)]
#![no_std]
#![crate_type = "staticlib"]
#![feature(lang_items)]

extern crate core;

pub static TEST: [u32; 3] = [
    0,
    1,
    2
];

#[no_mangle]
pub fn main() {
    let c = 0xabcd1234u32 as *mut u32;

    unsafe {
        *c = TEST[2];
    }
}

#[lang = "stack_exhausted"]
extern fn stack_exhausted() {}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
fn panic_fmt() -> ! { loop {} }
