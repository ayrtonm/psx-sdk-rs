//! This is a crate for developing homebrew for the Sony PlayStation 1.
//!
//! # Crate features
//!
//! Additional features can be enabled by building with
//! ```bash
//! cargo psx run --cargo-args "features psx/$FEATURE1,psx/$FEATURE2"
//! ```
//! Features may also be added in `Cargo.toml`. All features are disabled by
//! default.
//!
//! * `NA_region`/`EU_region`/`J_region` - Sets the region string in the [PS-EXE
//!   header](http://problemkaputt.de/psx-spx.htm#cdromfileformats).
//! * `min_panic` - Minimizes code generated for `panic!`s by removing error
//!   messages. Code is still generated to ensure that the processor hangs.
//! * `loadable_exe` - Allows returning from `main` to enable loading and
//!   unloading executables.
//! * `custom_oom` - Allows creating custom [allocation error handlers](https://github.com/rust-lang/rust/issues/51540)
//! * `heap` - Enables using [`heap!`][`heap!`] managed by [`linked_list_allocator`](https://crates.io/crates/linked_list_allocator).
//!   This is disabled by default to minimize dependencies for the default
//!   build.
//! * `nightlier` - For when nightly rustc isn't bleeding edge enough. This
//!   enables features requiring changes that aren't in upstream LLVM yet. Using
//!   this requires [building and patching LLVM](https://github.com/ayrtonm/psx-sdk-rs/tree/crates.io#building-the-compiler) as part of the rustc build.
//!   Currently this enables [`Atomic*`][core::sync::atomic] up to 16-bits and
//!   [`fences`][core::sync::atomic::compiler_fence].

#![no_std]
#![deny(missing_docs)]
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
// Used in psx::hw::irq
#![feature(variant_count)]
// Used for crate tests
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "main"]
#![cfg_attr(test, no_main)]

// This module is first since it defines the fuzz macros for tests
#[macro_use]
mod test;

pub mod dma;
mod framebuffer;
pub mod gpu;
#[doc(hidden)]
#[cfg(feature = "heap")]
pub mod heap;
pub mod hw;
mod macros;
mod panic;
#[doc(hidden)]
pub mod runtime;
#[doc(hidden)]
pub mod std;
pub mod sys;
pub mod tim;

/// Re-exported constants in a module for easy glob importing.
pub mod constants {
    const KB: usize = 1024;
    const MB: usize = 1024 * KB;
    /// The start of main RAM in KSEG0.
    pub const MAIN_RAM: *mut u32 = 0x8000_0000 as *mut u32;
    /// The size of main RAM.
    pub const MAIN_RAM_LEN: usize = 2 * MB;
    /// The start of the data cache.
    pub const DATA_CACHE: *mut u32 = 0x9F80_0000 as *mut u32;
    /// The size of the data cache.
    pub const DATA_CACHE_LEN: usize = 1 * KB;
    /// The size of the BIOS in RAM.
    pub const BIOS_LEN: usize = 64 * KB;
    pub use crate::gpu::colors::*;
    pub use crate::gpu::VideoMode::*;
}

/// Interrupt request types
pub mod irq {
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
}

#[cfg(not(feature = "custom_oom"))]
#[alloc_error_handler]
fn on_oom(layout: core::alloc::Layout) -> ! {
    panic!("Ran out of memory {:?}", layout);
}

pub use framebuffer::Framebuffer;
pub use tim::TIM;
