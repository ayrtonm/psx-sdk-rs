#![feature(lang_items)]
#![feature(no_std,core)]
#![no_std]
#![crate_type = "rlib"]
#![crate_name = "psx"]

extern crate core;

// Various lang items required by rustc
#[lang = "stack_exhausted"]
extern fn stack_exhausted() {}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
fn panic_fmt() -> ! { loop {} }
