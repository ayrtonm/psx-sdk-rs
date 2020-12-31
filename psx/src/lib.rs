//! Library for the original Sony PlayStation.
//!
//! This crate contains routines for using PSX peripherals, coprocessors and
//! memory-mapped I/O registers.
#![no_std]
#![deny(missing_docs)]
#![deny(warnings)]
// Required for BIOS function wrappers and coprocessors.
#![feature(asm, naked_functions)]
// Required for allocator error handling.
#![feature(alloc_error_handler)]
// Required for panic messages
#![feature(panic_info_message, fmt_as_str)]
// Pretty much required for this crate.
#![feature(min_const_generics)]
// Required for global variable workaround.
#![feature(once_cell, const_fn_fn_ptr_basics)]
// Const features used to increase the potential scope of const testing.
#![feature(
    const_ptr_offset,
    const_mut_refs,
    const_slice_from_raw_parts,
    const_raw_ptr_deref,
    const_int_pow
)]
// Could be removed if necessary.
#![feature(array_map)]
#![feature(unsafe_cell_get_mut)]
#![feature(exclusive_range_pattern)]

// These are internally used modules.
mod allocator;
mod builtins;
mod panic;

// These are the lowest-level public modules.
/// Wrappers for BIOS functions.
pub mod bios;
/// Traits for accessing coprocessor and memory-mapped I/O registers.
pub mod value;
/// Routines for including data from external files.
#[macro_use]
mod include;
/// Workarounds for global variables and panic-less functions.
#[macro_use]
pub mod workarounds;

// These are slightly higher level public modules in that they make use of the
// `value` module.
/// Coprocessor 0 registers and routines.
pub mod cop0;
/// Graphics transformation engine (coprocessor 2) routines.
pub mod gte;
/// Traits for addressing memory-mapped I/O registers.
pub mod mmio;

// These correspond to the different types of I/O registers and make use of the
// `mmio` module.
/// DMA routines.
pub mod dma;
/// GPU routines.
pub mod gpu;
/// Interrupt routines.
pub mod interrupt;
/// Interrupt request masking, acknowledge and wait routines.
pub mod irq;
/// Timer routines.
pub mod timer;

// This uses the I/O register methods to provide PSY-Q-compatible functions
// where it makes sense.
/// Routines from PSY-Q/PSn00bSDK
pub mod compatibility;

// These modules are roughly at the same level of abstraction as
// `compatibility`, but take different approaches to appear more
// high-level/ergonomic.

/// Framebuffer routines.
pub mod framebuffer;
/// General routines intended to be alternatives to the PSY-Q routines.
pub mod general;
/// Ordering table and primitive buffer routines.
pub mod graphics;
/// Printing routines.
pub mod printer;
/// Parsing texture data in TIM format.
pub mod tim;
/// Method for unzipping files.
pub mod unzip;

/// Used for testing only.
pub fn delay(n: u32) {
    for _ in 0..n {
        unsafe {
            core::ptr::read_volatile(0 as *mut u32);
        }
    }
}
