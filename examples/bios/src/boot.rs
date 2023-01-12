use crate::allocator::HEAP;
use crate::exceptions::exception_vec;
use crate::global::Global;
use crate::handlers::{a0_fn_vec, b0_fn_vec, c0_fn_vec};
use crate::main;
use crate::println;
use core::arch::asm;
use core::intrinsics::{volatile_copy_nonoverlapping_memory, volatile_set_memory};
use core::mem::size_of;
use psx::constants::KB;
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
         la $fp, {init_sp}
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
    unsafe {
        volatile_copy_nonoverlapping_memory(A0_VEC as *mut u32, a0_fn_vec as *const u32, 4);
        volatile_copy_nonoverlapping_memory(B0_VEC as *mut u32, b0_fn_vec as *const u32, 4);
        volatile_copy_nonoverlapping_memory(C0_VEC as *mut u32, c0_fn_vec as *const u32, 4);
    }

    println!("Wrote BIOS fn vectors. Debug output should now work.");
    unsafe {
        volatile_copy_nonoverlapping_memory(
            RAM_EXCEPTION_VEC as *mut u32,
            exception_vec as *const u32,
            4,
        );
    }
    println!("Wrote RAM exception vector");
}

fn init_ram() {
    extern "C" {
        // The linker script is set up so that these refer to the load addresses
        static __data_start: u32;
        static __data_end: u32;
        static mut __bss_start: u32;
        static __bss_end: u32;
    }
    unsafe {
        let start = &__data_start as *const u32 as usize;
        let end = &__data_end as *const u32 as usize;
        let len = (end - start) / 4;
        let dst = (KSEG0_BASE + 0x100) as *mut u32;
        let src = &__data_start as *const u32;
        println!(
            "Copying {} words from {:p} to {:p} to initialize .data",
            len, src, dst
        );
        volatile_copy_nonoverlapping_memory(dst, src, len);

        let bss_start = &__bss_start as *const u32 as usize;
        let bss_end = &__bss_end as *const u32 as usize;
        let bss_len = (bss_end - bss_start) / 4;
        let bss_dst = &mut __bss_start as *mut u32;
        println!(
            "Zeroing out {} words from {:x} to {:x} to initialize .bss",
            bss_len, bss_start, bss_end
        );
        volatile_set_memory(bss_dst, 0, bss_len);
    }

    const HEAP_SIZE: usize = 4 * KB / size_of::<u32>();
    static HEAP_MEM: Global<[u32; HEAP_SIZE]> = Global::new([0; HEAP_SIZE]);
    unsafe {
        let ptr = HEAP_MEM.as_mut().as_mut_ptr().cast();
        let len = HEAP_MEM.as_mut().len() * size_of::<u32>();
        println!("Initializing the heap at {:p} ({} bytes)", ptr, len);
        HEAP.as_mut().init(ptr, len);
    }
}
