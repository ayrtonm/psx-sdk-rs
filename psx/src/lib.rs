#![no_std]
// Pretty much required to implement certain things
#![feature(min_const_generics)]
// Used for error messages
#![feature(panic_info_message, fmt_as_str, alloc_error_handler)]
// Could probably get away with not using these if necessary
#![feature(bool_to_option, array_map, type_alias_impl_trait)]
// Only used for bios trampolines so far
#![feature(asm, naked_functions)]
// TODO: I should start using this eventually
//#![deny(missing_docs)]

mod allocator;
mod builtins;
#[macro_use]
mod macros;
mod panic;
#[macro_use]
mod value;

pub mod bios;
pub mod cop0;
pub mod dma;
pub mod gpu;
pub mod gte;
pub mod interrupt;
pub mod mmio;
pub mod tim;
pub mod timers;

pub mod framebuffer;
pub mod graphics;
pub mod printer;
pub mod unzip;

pub fn delay(n: u32) {
    for _ in 0..n {
        unsafe {
            core::ptr::read_volatile(0 as *mut u32);
        }
    }
}
