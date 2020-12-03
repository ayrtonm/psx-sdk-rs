#![no_std]
// Pretty much required to implement certain things
#![feature(min_const_generics)]
// Could probably get away with not using these if necessary
#![feature(bool_to_option, array_map, type_alias_impl_trait)]
// Only used for bios trampolines so far
#![feature(asm, naked_functions)]
#![feature(alloc_error_handler)]

mod allocator;
pub mod bios;
mod builtins;
pub mod dma;
pub mod framebuffer;
pub mod gpu;
pub mod interrupt;
#[macro_use]
pub mod macros;
pub mod mmio;
mod panic;
pub mod tim;
pub mod unzip;

pub fn delay(n: u32) {
    for _ in 0..n {
        unsafe {
            core::ptr::read_volatile(0 as *mut u32);
        }
    }
}

#[macro_export]
macro_rules! exe {
    () => {
        use psx::mmio::MMIO;

        mod __exe__ {
            #[no_mangle]
            fn main() {
                let mmio = unsafe { psx::mmio::MMIO::new() };
                super::main(mmio)
            }
        }
    };
}
