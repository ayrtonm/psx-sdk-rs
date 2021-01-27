//! This is a crate for developing homebrew for the original Sony PlayStation.
#![no_std]
#![warn(missing_docs)]
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
// Required to use `illegal` in const fn
#![feature(const_unreachable_unchecked, const_panic)]
//// Used to approximate sine and cosine.
//#![feature(const_fn_floating_point_arithmetic, const_float_bits_conv)]
// Could be removed if necessary.
#![feature(array_map)]
#![feature(unsafe_cell_get_mut)]

use core::hint::unreachable_unchecked;
use core::mem::size_of;

#[macro_use]
mod include;

mod allocator;
mod builtins;
#[macro_use]
mod std;
mod panic;
#[macro_use]
mod test;

/// Wrappers for calling BIOS functions.
pub mod bios;
/// DMA channels and transfers.
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

const fn illegal() -> ! {
    if cfg!(feature = "forbid_UB") {
        panic!("")
    } else {
        unsafe { unreachable_unchecked() }
    }
}

const fn num_words<T>() -> usize {
    size_of::<T>() / 4
}
