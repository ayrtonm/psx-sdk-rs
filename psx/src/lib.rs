//! This is a crate for developing homebrew for the Sony PlayStation 1.
#![no_std]
#![forbid(missing_docs)]
// For the `AsCStr` trait
#![feature(core_c_str)]
// Used to make `AsCStr` efficient
#![feature(
    maybe_uninit_uninit_array,
    maybe_uninit_slice,
    maybe_uninit_write_slice
)]
// Used to implement `ImplsAsCStr` trait
#![feature(min_specialization)]
// For the panic handler
#![feature(panic_info_message)]
// For global_asm! on MIPS
#![feature(asm_experimental_arch)]
// For `__start`'s return type
#![feature(never_type)]
// For BIOS OOM messages
#![feature(alloc_error_handler)]
// Used for crate tests
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "main"]
#![cfg_attr(test, no_main)]

// This module is first since it defines the fuzz macros for tests
#[macro_use]
mod test;

mod panic;
#[doc(hidden)]
pub mod runtime;
#[doc(hidden)]
pub mod std;
pub mod sys;
