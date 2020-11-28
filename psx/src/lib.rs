#![no_std]
#![feature(min_const_generics, type_alias_impl_trait, bool_to_option, array_map)]

mod builtins;
pub mod dma;
pub mod framebuffer;
pub mod gpu;
pub mod interrupt;
pub mod mmio;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
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
