//! This is a crate for developing homebrew for the original Sony PlayStation.
#![no_std]
// Only allowed for const generics
#![allow(incomplete_features)]
// TODO: Reconsider whether to enable this incomplete feature after wrapping up this refactor.
// Ideally I'll only be using the basics of `const_generics`, but it might make debugging more
// tricky if it breaks anything else.
#![feature(const_generics)]
//#![warn(missing_docs)]
// Required for BIOS function wrappers and coprocessors.
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(c_variadic)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
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
#![feature(array_map, variant_count)]
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
#[doc(hidden)]
pub mod unzip;

pub mod hal;

pub mod dma;
pub mod gpu;
pub mod interrupt;
pub mod timer;

#[macro_use]
#[doc(hidden)]
pub mod tty;
#[macro_use]
pub mod bios;

mod allocator;
mod builtins;
mod panic;
mod runtime;
mod test;

pub mod graphics;
pub mod tim;

pub mod framebuffer;
pub mod printer;
