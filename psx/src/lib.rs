//! Library for the original Sony PlayStation.
//!
//! This crate contains routines for using PSX peripherals, coprocessors and
//! memory-mapped I/O registers.
#![no_std]
#![deny(missing_docs)]
#![deny(warnings)]
// Allowed to match nomenclature in [nocash specs](http://problemkaputt.de/psx-spx.htm).
#![allow(non_upper_case_globals)]
// Required for BIOS function wrappers and coprocessors.
#![feature(asm, naked_functions)]
// Required for allocator error handling.
#![feature(alloc_error_handler)]
#![feature(min_const_generics)]

mod allocator;
mod panic;

/// Wrappers for BIOS functions.
pub mod bios;
/// Traits for accessing coprocessor and memory-mapped I/O registers.
pub mod value;

/// Coprocessor 0 registers and routines.
pub mod cop0;
/// Traits for addressing memory-mapped I/O registers.
pub mod mmio;

/// Interrupt routines.
pub mod interrupt;
/// DMA routines.
pub mod dma;
