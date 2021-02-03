//! This is a crate for developing homebrew for the original Sony PlayStation.
#![no_std]
//#![warn(missing_docs)]
// Required for BIOS function wrappers and coprocessors.
#![feature(global_asm)]
#![feature(c_variadic)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message, fmt_as_str)]
// Const features used to increase the potential scope of const testing.
#![feature(
    const_ptr_offset,
    const_mut_refs,
    const_slice_from_raw_parts,
    const_raw_ptr_deref,
    const_fn_fn_ptr_basics,
    const_fn
)]
// Required for const `illegal`
#![feature(const_unreachable_unchecked, const_panic)]
// Could be removed if necessary.
#![feature(array_map)]
// Required to test psx crate
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "test_main"]
#![cfg_attr(test, no_main)]

macro_rules! as_array {
    ($msg:literal) => {
        unsafe { *($msg.as_ptr() as *const _) }
    };
}

#[cfg(any(feature = "NA_region", test))]
#[no_mangle]
#[doc(hidden)]
#[link_section = ".region"]
pub static _REGION: [u8; 55] = as_array!("Sony Computer Entertainment Inc. for North America area");

#[cfg(feature = "EU_region")]
#[no_mangle]
#[doc(hidden)]
#[link_section = ".region"]
pub static _REGION: [u8; 48] = as_array!("Sony Computer Entertainment Inc. for Europe area");

#[cfg(feature = "J_region")]
#[no_mangle]
#[doc(hidden)]
#[link_section = ".region"]
pub static _REGION: [u8; 47] = as_array!("Sony Computer Entertainment Inc. for Japan area");

#[no_mangle]
#[doc(hidden)]
#[link_section = ".psx_exe"]
pub static _PSX_EXE: [u8; 8] = as_array!("PS-X EXE");

#[macro_use]
mod include;
#[macro_use]
mod std;
#[macro_use]
pub mod bios;

mod allocator;
mod builtins;
mod panic;
mod runtime;
mod test;

pub mod dma;
pub mod framebuffer;
pub mod gpu;
pub mod graphics;
pub mod hal;
pub mod interrupt;
pub mod printer;
pub mod tim;
pub mod timer;
pub mod unzip;
