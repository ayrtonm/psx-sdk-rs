//! Library for the original Sony PlayStation.
#![no_std]
// Required for BIOS function wrappers and coprocessors.
#![feature(asm, naked_functions)]
// Pretty much required for this crate.
#![feature(min_const_generics)]
/*
// Required for allocator error handling.
#![feature(alloc_error_handler)]
// Required for panic messages
#![feature(panic_info_message, fmt_as_str)]
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
// Const feature to improve performance at the risk of UB while using const testing.
#![feature(const_unreachable_unchecked)]
// Used to approximate sine and cosine.
#![feature(const_fn_floating_point_arithmetic, const_float_bits_conv)]
// Could be removed if necessary.
#![feature(array_map)]
#![feature(bool_to_option)]
#![feature(unsafe_cell_get_mut)]
#![feature(exclusive_range_pattern)]
*/

/// Wrappers for BIOS functions.
pub mod bios;
pub mod dma;
/// A hardware abstraction layer for memory-mapped I/O registers and
/// coprocessors.
pub mod hal;

mod builtins;
mod panic;

fn illegal() -> ! {
    if cfg!(feature = "forbid_UB") {
        unreachable!("")
    } else {
        unsafe { core::hint::unreachable_unchecked() }
    }
}

fn main() {
    let mut gpu_dma = dma::GPU::new();
    let mut ar = [0; 10];
    let mut ar2 = [1; 11];
    ar[0] = 1;
    gpu_dma.prepare();
    let transfer = gpu_dma.send(&ar);
    transfer.wait();
    let transfer2 = gpu_dma.send(&ar2);
    ar[1] = 2;
}
