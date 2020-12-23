//! IO routines for the original Sony PlayStation.
//!
//! This crate contains routines for using PSX peripherals and coprocessors.
#![no_std]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_upper_case_globals)]
// Required for BIOS function wrappers.
#![feature(asm, naked_functions)]
// Required for allocator error handling.
#![feature(alloc_error_handler)]
#![feature(min_const_generics)]

mod allocator;
mod panic;

/// Wrappers for BIOS functions.
pub mod bios;

/// Methods for accessing coprocessor and memory-mapped I/O registers.
pub mod value;

/// Coprocessor 0 routines.
pub mod cop0;
/// Traits for accessing memory-mapped I/O registers.
pub mod mmio;

/// Interrupt routines.
pub mod interrupt;

/// DMA channel routines.
pub mod dma;
