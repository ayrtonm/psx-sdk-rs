#![no_std]
#![feature(min_const_generics, type_alias_impl_trait, bool_to_option, array_map)]
// Only used for bios trampolines so far
#![feature(asm, naked_functions)]

pub mod bios;
mod builtins;
pub mod dma;
pub mod framebuffer;
pub mod gpu;
pub mod interrupt;
#[macro_use]
pub mod macros;
pub mod mmio;
pub mod tim;
pub mod unzip;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

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
