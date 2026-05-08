//! BIOS function wrappers.
//!
//! This module contains wrappers for functions provided by the BIOS.

use crate::CriticalSection;
use core::ffi::CStr;

pub mod fs;
pub mod gamepad;
pub mod heap;
pub mod kernel;
pub mod rng;
pub mod tty;

/// Calls the given function in an interrupt-free critical section using BIOS
/// syscalls.
///
/// # Safety
///
/// Exception handlers might not support nested exceptions so make sure to not
/// call this from a critical section.
pub unsafe fn critical_section<F: FnMut(&mut CriticalSection) -> R, R>(mut f: F) -> R {
    let changed_state = unsafe { kernel::psx_enter_critical_section() };
    // SAFETY: We are in a critical section so we can create this
    let mut cs = unsafe { CriticalSection::new() };
    let res = f(&mut cs);
    if changed_state {
        unsafe {
            kernel::psx_exit_critical_section();
        }
    };
    res
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

enum SystemRegion {
    Japan,
    NorthAmerica,
    Europe,
    Unknown,
}

/// Returns the system's region.
pub fn get_system_region() -> SystemRegion {
    const REGION_ID: *const u8 = 0xbfc7ff52 as _;

    // SAFETY: Sony BIOSes always have a character at this pointer denoting the
    // region of the console.
    //                                   pointer --
    //                                            v
    // SCPH-5500: System ROM Version 3.0 09/09/96 J
    // SCPH-5501: System ROM Version 3.0 11/18/96 A
    // SCPH-5502: System ROM Version 3.0 01/06/97 E
    // etc.
    let region_char = char::from(unsafe { REGION_ID.read() });

    match region_char {
        'J' => SystemRegion::Japan,
        'A' => SystemRegion::NorthAmerica,
        'E' => SystemRegion::Europe,
        _ => SystemRegion::Unknown,
    }
}
/// Returns the kernel's date in BCD (e.g. 0x19951204).
pub fn get_system_date() -> u32 {
    unsafe { kernel::psx_get_system_info(0) }
}
