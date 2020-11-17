#![no_std]
#![feature(core_intrinsics)]
#![feature(alloc_error_handler)]
// These are not strictly necessary for writing a std library for the PSX, but they simplify things
#![feature(min_const_generics)]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(doc_cfg)]

pub mod allocator;
pub mod bios;
mod builtins;
pub mod dma;
pub mod gpu;
pub mod interrupt;
pub mod io;
mod registers;

use core::intrinsics::volatile_load;
use core::panic::PanicInfo;

pub fn delay(n: u32) {
    for _ in 0..n {
        unsafe {
            volatile_load(0 as *mut u32);
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(doc)]
use crate::io::IO;
#[cfg(doc)]
pub fn main(mut io: IO) {}

#[macro_export]
macro_rules! exe {
    () => {
        psx::exe!(0x1FA0_0000, 2048 * 1024);
    };
    (no heap) => {
        psx::exe!(0, 0);
    };
    (big heap) => {
        psx::exe!(0x1F00_0000, 8192 * 1024);
    };
    (fast heap) => {
        psx::exe!(0x1F80_0000, 1024);
    };
    ($heap_addr:expr, $heap_size:expr) => {
        //// TODO: fix the linker error with rust-lld
        //extern crate alloc;
        //// TODO: add other common collections here
        //use alloc::borrow::Cow;
        //use alloc::boxed::Box;
        //use alloc::rc::Rc;
        //use alloc::vec::Vec;

        use psx::io::IO;

        mod executable {
            //TODO: remove link_section (or change to .text) for regular .psexe's
            //#[link_section = ".exe"]
            #[no_mangle]
            fn main() {
                let io = unsafe { psx::io::IO::new() };
                if $heap_size != 0 {
                    psx::bios::init_heap($heap_addr, $heap_size);
                }
                super::main(io)
            }
        }
    };
}
