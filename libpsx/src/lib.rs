#![no_std]
#![feature(core_intrinsics)]
// These are not strictly necessary for writing a std library for the PSX, but they simplify things
#![feature(min_const_generics)]

pub mod gpu;
mod context;
mod macros;

pub use context::IOCX;

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { }
}
