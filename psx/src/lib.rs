//! This is a crate for developing homebrew for the Sony PlayStation 1.
#![no_std]
// Used for const constraints (e.g. Packet::new)
#![allow(path_statements)]
//#![deny(missing_docs)]
// Hacky workaround for the lack of compiler fences in llvm's MIPS-I codegen
#![feature(bench_black_box)]
// Used to implement PrintfArg trait
#![feature(min_specialization)]
// Required for global_asm! on MIPS
#![feature(asm_experimental_arch)]
// For an efficient `AsCStr` implementation
#![feature(
    maybe_uninit_uninit_array,
    maybe_uninit_write_slice,
    maybe_uninit_slice
)]
// For the panic handler
#![feature(panic_info_message)]
// For BIOS OOM messages
#![feature(alloc_error_handler)]
// Used in irq
#![feature(variant_count)]
// For the exception-handler installer
#![feature(asm_sym)]
// For thread arguments
#![feature(naked_functions)]
// For the initial thread's return type
#![feature(never_type)]
// For crate tests
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "main"]
#![cfg_attr(test, no_main)]

use core::arch::asm;
use core::mem::size_of;
use core::slice;

#[macro_use]
mod test;

pub mod dma;
pub mod gpu;
#[doc(hidden)]
pub mod graphics;
pub mod runtime;
// TODO: Add cfc2 and ctc2 to LLVM to enable this
//pub mod gte;
#[doc(hidden)]
pub mod heap;
pub mod hw;
// The `std` module should be public but hidden since `as_cstr` is used from
// macros which may be in user crates.
mod framebuffer;
mod panic;
#[doc(hidden)]
pub mod std;
pub mod sys;
pub mod tim;

pub const KUSEG: usize = 0x0000_0000;
pub const KSEG0: usize = 0x8000_0000;
pub const CACHE: usize = 0x9F80_0000;

/// Returns a mutable slice to the data cache.
pub unsafe fn data_cache<'a>() -> &'a mut [u32] {
    let ptr = CACHE as *mut u32;
    let len = 1024 / size_of::<u32>();
    slice::from_raw_parts_mut(ptr, len)
}

/// Returns a mutable slice to the unused part of main RAM.
///
/// This function uses the current stack pointer to determine the end of the
/// unused part of RAM, so its length may vary between callsites. It also
/// assumes that there is a single contiguous region of unused memory which
/// starts at the `__heap_start` symbol. The start of the slice is rounded up to
/// a multiple of 4 by the linker script, so it may miss up to 3 bytes of unused
/// memory. However, this allows returning a `u32` slice which is typically more
/// convenient.
///
/// # Safety
///
/// It is the caller's responsibility to ensure that the stack does not increase
/// over the lifetime of the  return value. Failure to do so is **undefined
/// behavior**. Note that it is possible to split the return value using
/// [`split_at_mut`][`slice::split_at_mut()`] or similar, then drop the upper
/// part of the slice to create more space for the stack to grow without causing
/// undefined behavior.
pub unsafe fn unused_memory<'a>() -> &'a mut [u32] {
    extern "C" {
        static mut __heap_start: u32;
    }
    // SAFETY: This symbol is defined by the linker script
    let ptr = &mut __heap_start as *mut u32;
    let sp: usize;
    asm! {
        ".set noat
         move {}, $29", out(reg) sp
    };
    let len = sp - ptr as usize;
    slice::from_raw_parts_mut(ptr, len)
}

/// An interrupt request
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IRQ {
    /// vertical blank interrupt request (NTSC = 60Hz, PAL = 50Hz)
    Vblank = 0,
    /// GPU interrupt requested via the GP0(1Fh) command
    GPU,
    /// CDROM interrupt request
    CDROM,
    /// DMA interrupt request
    DMA,
    /// Timer 0 (dot clock or sysclock)
    Timer0,
    /// Timer 1 (Hblank or sysclock)
    Timer1,
    /// Timer 2 (sysclock or fractional sysclock)
    Timer2,
    /// Controller and memory card byte received
    ControllerMemoryCard,
    /// Serial IO port
    SIO,
    /// Sound processing unit
    SPU,
    /// Secondary controller interrupt request
    ControllerPIO,
}

pub use crate::gpu::packet::{link_list, ordering_table};
pub use framebuffer::{draw_sync, enable_vblank, vsync, Framebuffer};
pub use graphics::fixed_point::F16;
pub use graphics::{cos, f16, sin, Vf, Vi, FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8, PI};
pub use heap::{critical_section, Global};
