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
#[macro_use]
pub mod unzip;

use core::intrinsics::volatile_load;
use core::panic::PanicInfo;

pub fn delay(n: u32) {
    for _ in 0..n {
        unsafe {
            volatile_load(0 as *mut u32);
        }
    }
}

// TODO: There should be a better way to implement `slice_to_array` for u8, u16, u32, etc.
pub const fn u8_array<const N: usize>(slice: &[u8]) -> [u8; N] {
    let mut ar = [0; N];
    let mut i = 0;
    while i < N {
        ar[i] = slice[i];
        i += 1;
    }
    ar
}

pub const fn u32_array<const N: usize>(slice: &[u32]) -> [u32; N] {
    let mut ar = [0; N];
    let mut i = 0;
    while i < N {
        ar[i] = slice[i];
        i += 1;
    }
    ar
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
