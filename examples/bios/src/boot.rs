use crate::exceptions::exception_vec;
use crate::println;
use crate::{fn_vec, main};
use core::arch::asm;
use core::intrinsics::volatile_copy_nonoverlapping_memory;
use psx::constants::*;

// This is the entry point which is placed at 0xBFC0_0000 by the linker script
// since this is the only function .text.boot. The stack pointer is
// uninitialized so it must be a naked function.
#[naked]
#[no_mangle]
#[link_section = ".text.boot"]
unsafe extern "C" fn boot() -> ! {
    asm! {
        "la $sp, {init_sp}
         j start",
        init_sp = const(KSEG0_BASE + MAIN_RAM_LEN - 0x100),
        options(noreturn)
    }
}

// The stack pointer is now initialized so this doesn't have to be a naked
// function.
#[no_mangle]
extern "C" fn start() -> ! {
    // Write handlers to the BIOS fn and general exception vectors
    init_vectors();
    init_ram();
    main();
    // Hang if the main loop returns
    loop {}
}

fn init_vectors() {
    // Write to the fn vectors
    for vec in [A0_VEC, B0_VEC, C0_VEC] {
        unsafe {
            volatile_copy_nonoverlapping_memory(vec as *mut u32, fn_vec as *const u32, 4);
        }
    }

    println!("Wrote BIOS fn vectors. Debug output should now work.");
    unsafe {
        volatile_copy_nonoverlapping_memory(
            RAM_EXCEPTION_VEC as *mut u32,
            exception_vec as *const u32,
            6,
        );
    }
    println!("Wrote RAM exception vector");
}

fn init_ram() {
    extern "C" {
        // The linker script is set up so that these refer to the load addresses
        static __data_start: u32;
        static __data_end: u32;
    }
    unsafe {
        let start = &__data_start as *const u32 as usize;
        let end = &__data_end as *const u32 as usize;
        let len = end - start;
        volatile_copy_nonoverlapping_memory(
            (KSEG0_BASE + 0x100) as *mut u32,
            &__data_start as *const u32,
            len,
        );
    }
}
