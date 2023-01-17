//! This is a crate for developing homebrew for the Sony PlayStation 1.
//!
//! # Crate features
//!
//! Additional features can be enabled by building with
//! ```bash
//! cargo psx run --features psx/$FEATURE1,psx/$FEATURE2
//! ```
//! Features may also be added in `Cargo.toml`. All features are disabled by
//! default.
//!
//! * `NA_region`/`EU_region`/`J_region` - Sets the region string in the [PS-EXE
//!   header](http://problemkaputt.de/psx-spx.htm#cdromfileformats).
//! * `min_panic` - Minimizes code generated for `panic!`s by printing error
//!   messages to stdout.
//! * `no_panic` - Further minimizes code generated for `panic!`s by omitting
//!   panic error messages. Code is still generated to ensure that panics hang
//!   the processor.
//! * `loadable_exe` - Allows returning from `main` to enable loading and
//!   unloading executables.
//! * `custom_oom` - Allows creating custom [allocation error handlers](https://github.com/rust-lang/rust/issues/51540)
//! * `heap` - Enables using [`heap!`][`heap!`] managed by [`linked_list_allocator`](https://crates.io/crates/linked_list_allocator).
//!   This is disabled by default to minimize dependencies for the default
//!   build.
//! * `nightlier` - For when nightly rustc isn't bleeding edge enough. This
//!   enables features requiring changes that aren't in upstream LLVM yet. Using
//!   this requires [building and patching LLVM](https://github.com/ayrtonm/psx-sdk-rs/tree/master/patches#rustc-build-instructions) as part of the rustc build.
//!   Currently this enables [`Atomic*`][core::sync::atomic] up to 16-bits and
//!   [`fences`][core::sync::atomic::compiler_fence].

#![no_std]
#![deny(missing_docs)]
// For compile-time Wavefront OBJ parser
#![feature(const_mut_refs, maybe_uninit_array_assume_init)]
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
// Used in psx::hw::irq
#![feature(variant_count)]
// Used in psx::sys::fs
#![feature(cstr_from_bytes_until_nul, pointer_is_aligned)]
// Used for crate tests
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "main"]
#![cfg_attr(test, no_main)]

// This module is first since it defines the fuzz macros for tests
#[macro_use]
mod test;

pub mod dma;
pub mod format;
mod framebuffer;
pub mod gpu;
#[doc(hidden)]
pub mod heap;
pub mod hw;
mod macros;
pub mod math;
mod panic;
#[doc(hidden)]
pub mod runtime;
#[doc(hidden)]
pub mod std;
pub mod sys;

/// Re-exported constants in a module for easy glob importing.
pub mod constants {
    /// A kilobyte
    pub const KB: usize = 1024;

    /// A megabyte
    pub const MB: usize = 1024 * KB;

    /// The start of main RAM in KUSEG.
    pub const KUSEG: usize = 0x0000_0000;
    /// The start of main RAM in KSEG0.
    pub const KSEG0: usize = 0x8000_0000;
    /// The start of main RAM in KSEG1.
    pub const KSEG1: usize = 0xA000_0000;

    /// The size of main RAM.
    pub const MAIN_RAM_LEN: usize = 2 * MB;
    /// The size of the BIOS in RAM.
    pub const BIOS_LEN: usize = 64 * KB;

    /// The BIOS A0 function vector
    pub const A0_VEC: usize = 0x8000_00A0;
    /// The BIOS B0 function vector
    pub const B0_VEC: usize = 0x8000_00B0;
    /// The BIOS C0 function vector
    pub const C0_VEC: usize = 0x8000_00C0;

    /// The general exception vector in RAM
    pub const RAM_EXCEPTION_VEC: usize = 0x8000_0080;
    /// The general exception vector in ROM
    pub const ROM_EXCEPTION_VEC: usize = 0xBFC0_0180;

    /// The start of the data cache.
    pub const DATA_CACHE: usize = 0x9F80_0000;
    /// The size of the data cache.
    pub const DATA_CACHE_LEN: usize = 1 * KB;

    /// The entrypoint of the post-boot function.
    pub const POST_BOOT_ENTRYPOINT: usize = 0x9F00_0000;
    /// The entrypoint of the pre-boot function.
    pub const PRE_BOOT_ENTRYPOINT: usize = 0x9F00_0080;

    pub use crate::gpu::colors::*;
    pub use crate::gpu::VideoMode::*;
    pub use crate::math::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8, PI};
    pub use crate::sys::gamepad::buttons::*;
}

/// Interrupt request types
pub mod irq {
    /// An interrupt request
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    /// All the interrupt requests from the lowest bit to the highest
    pub const ALL_IRQS: [IRQ; 11] = [
        IRQ::Vblank,
        IRQ::GPU,
        IRQ::CDROM,
        IRQ::DMA,
        IRQ::Timer0,
        IRQ::Timer1,
        IRQ::Timer2,
        IRQ::ControllerMemoryCard,
        IRQ::SIO,
        IRQ::SPU,
        IRQ::ControllerPIO,
    ];
}

#[cfg(not(feature = "custom_oom"))]
#[alloc_error_handler]
fn on_oom(layout: core::alloc::Layout) -> ! {
    panic!("Ran out of memory {:?}", layout);
}

pub use framebuffer::{Framebuffer, LoadedTIM, TextBox};
//pub use format::tim::{Bitmap, TIMError, TIM};

/// A token ensuring that code is being executed in a critical section.
pub struct CriticalSection(());

impl CriticalSection {
    /// Creates a new critical section token
    ///
    /// # SAFETY: Since owning a CriticalSection means we are in a critical
    /// section, this is only safe to create if we are actually in a critical
    /// section.
    pub unsafe fn new() -> Self {
        Self(())
    }
}
