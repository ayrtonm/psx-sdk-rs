#![no_std]
// Pretty much required to implement certain things
#![feature(min_const_generics)]
// TODO: Is there an alternative to access the panic message or work with the payload instead?
#![feature(panic_info_message, fmt_as_str)]
// Could probably get away with not using these if necessary
#![feature(bool_to_option, array_map, type_alias_impl_trait)]
// Only used for bios trampolines so far
#![feature(asm, naked_functions)]
#![feature(alloc_error_handler)]

mod allocator;
pub mod bios;
mod builtins;
pub mod cop0;
pub mod dma;
pub mod framebuffer;
pub mod gpu;
pub mod interrupt;
#[macro_use]
mod macros;
pub mod mmio;
mod panic;
pub mod printer;
pub mod tim;
pub mod unzip;

pub fn delay(n: u32) {
    for _ in 0..n {
        unsafe {
            core::ptr::read_volatile(0 as *mut u32);
        }
    }
}
