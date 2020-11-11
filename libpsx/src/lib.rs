#![no_std]
#![feature(core_intrinsics)]
// These are not strictly necessary for writing a std library for the PSX, but they simplify things
#![feature(min_const_generics)]

pub mod gpu;
mod context;
mod macros;

use core::intrinsics::volatile_load;
use core::panic::PanicInfo;

pub use context::IOCX;

pub fn delay(n: u32) {
    for _ in 0..n {
        unsafe {
            volatile_load(0 as *mut u32);
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { }
}
