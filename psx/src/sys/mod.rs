//! BIOS function wrappers.
//!
//! This modules contains wrappers for functions provided by the BIOS.

use crate::KSEG0;
use core::mem::size_of;

pub mod fs;
pub mod gamepad;
pub mod heap;
pub mod kernel;
pub mod tty;
pub mod rng;

fn table_of_tables() -> *const u32 {
    let addr = KSEG0 + (0x100 / size_of::<u32>());
    addr as *const u32
}

/// Runs the given function in an interrupt-free critical section using BIOS
/// syscalls.
pub fn critical_section<F: FnOnce() -> R, R>(f: F) -> R {
    unsafe {
        kernel::enter_critical_section();
        let res = f();
        kernel::exit_critical_section();
        res
    }
}
