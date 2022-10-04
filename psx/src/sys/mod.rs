//! BIOS function wrappers.
//!
//! This module contains wrappers for functions provided by the BIOS.

use core::ffi::CStr;

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
        if kernel::psx_enter_critical_section() {
            let res = f();
            kernel::psx_exit_critical_section();
            res
        } else {
            f()
        }
    }
}

/// Returns the kernel's version string.
pub fn get_system_version() -> &'static CStr {
    // SAFETY: Calling get_system_info with index 2 gives a pointer with a
    // static lifetime to the version string. There are no safety requirement.
    let version = unsafe { kernel::psx_get_system_info(2) as *const i8 };
    // SAFETY: Let's hope the BIOS returned a pointer to a null-terminated string
    // to its own memory.
    unsafe { CStr::from_ptr(version) }
}

/// Returns the kernel's date in BCD (e.g. 0x19951204).
pub fn get_system_date() -> u32 {
    unsafe { kernel::psx_get_system_info(0) }
}
