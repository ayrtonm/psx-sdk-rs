//! BIOS function wrappers.
//!
//! This module contains wrappers for functions provided by the BIOS.

pub mod fs;
pub mod gamepad;
pub mod heap;
pub mod kernel;
pub mod rng;
pub mod tty;

/// Calls the given function in an interrupt-free critical section using BIOS
/// syscalls.
pub fn critical_section<F: FnOnce() -> R, R>(f: F) -> R {
    unsafe {
        if kernel::enter_critical_section() {
            let res = f();
            kernel::exit_critical_section();
            res
        } else {
            f()
        }
    }
}
