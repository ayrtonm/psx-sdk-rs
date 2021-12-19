//! This is a crate for developing homebrew for the Sony PlayStation 1.
#![no_std]
//#![deny(missing_docs)]
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
use core::slice;

pub mod dma;
pub mod gpu;
pub mod hw;
pub mod irq;
// The `std` module should be public but hidden since `as_cstr` is used from
// macros which may be in user crates.
#[doc(hidden)]
pub mod std;
pub mod sys;
mod test;

// This is the crate-wide fallback for artisanally-crafted errors for each function.
pub type Result<T> = core::result::Result<T, &'static str>;

pub const KUSEG: usize = 0x0000_0000;
pub const KSEG0: usize = 0x8000_0000;

/// The runtime used by the default linker scripts.
#[no_mangle]
extern "C" fn _start() -> ! {
    // SAFETY: If there is no unmangled function named `main` this causes an error
    // at link-time.
    unsafe {
        #[cfg(not(test))]
        extern "Rust" {
            fn main();
        }
        main();
    }
    panic!("`main` should not return")
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    match info.location() {
        Some(location) => {
            println!(
                "Panicked at {}:{}:{}",
                location.file(),
                location.line(),
                location.column()
            )
        },
        None => {
            println!("Panicked at unknown location")
        },
    }
    if let Some(msg) = info.message() {
        println!("{}", msg)
    }
    loop {}
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
pub unsafe fn free_memory<'a>() -> &'a mut [u32] {
    extern "C" {
        static mut __heap_start: u32;
    }
    // SAFETY: This symbol is defined by the linker script
    let ptr = unsafe { &mut __heap_start as *mut u32 };
    let sp: usize;
    asm! {
        ".set noat
         move {}, $29", out(reg) sp
    };
    let len = sp - ptr as usize;
    slice::from_raw_parts_mut(ptr, len)
}

// Define string-literals to embed in PSEXE header
// Using the same identifier for all regions conveniently makes the crate
// features mutually exclusive
macro_rules! as_array {
    ($msg:literal) => {
        // SAFETY: This dereferences a pointer to a literal which has a static lifetime.
        unsafe { *($msg.as_ptr() as *const _) }
    };
}

#[cfg(any(feature = "NA_region", test))]
#[used]
#[no_mangle]
#[doc(hidden)]
#[link_section = ".region"]
pub static _REGION: [u8; 55] = as_array!("Sony Computer Entertainment Inc. for North America area");

#[cfg(feature = "EU_region")]
#[used]
#[no_mangle]
#[doc(hidden)]
#[link_section = ".region"]
pub static _REGION: [u8; 48] = as_array!("Sony Computer Entertainment Inc. for Europe area");

#[cfg(feature = "J_region")]
#[used]
#[no_mangle]
#[doc(hidden)]
#[link_section = ".region"]
pub static _REGION: [u8; 47] = as_array!("Sony Computer Entertainment Inc. for Japan area");

#[used]
#[no_mangle]
#[doc(hidden)]
#[link_section = ".psx_exe"]
pub static _PSX_EXE: [u8; 8] = as_array!("PS-X EXE");
