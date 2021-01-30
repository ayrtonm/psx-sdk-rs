//! This is a crate for developing homebrew for the original Sony PlayStation.
#![no_std]
//#![warn(missing_docs)]
// Required for BIOS function wrappers and coprocessors.
#![feature(asm, naked_functions)]
// Required for many things in this crate.
#![feature(min_const_generics)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message, fmt_as_str)]
// Const features used to increase the potential scope of const testing.
#![feature(
    const_ptr_offset,
    const_mut_refs,
    const_int_pow,
    const_slice_from_raw_parts,
    const_raw_ptr_deref,
    const_fn_fn_ptr_basics,
    const_fn
)]
// Required for const `illegal`
#![feature(const_unreachable_unchecked, const_panic)]
// Could be removed if necessary.
#![feature(array_map)]
#![feature(unsafe_cell_get_mut)]
// Required to test psx crate
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "test_main"]
#![cfg_attr(test, no_main)]

#[macro_use]
mod include;
#[macro_use]
mod std;

mod allocator;
mod builtins;
mod panic;
mod runtime;
mod test;

pub mod bios;
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
