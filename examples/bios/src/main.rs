#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]
#![feature(naked_functions)]
#![feature(core_intrinsics)]

use core::arch::asm;
use core::intrinsics::volatile_copy_nonoverlapping_memory;
use psx::constants::*;
use psx::hw::cop0;
use psx::hw::cop0::IntSrc;
use psx::hw::irq;
use psx::hw::Register;
use psx::sys::kernel::*;

mod stdout;

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
unsafe extern "C" fn start() -> ! {
    // Write handlers to the BIOS fn and general exception vectors
    init_vectors();
    main_loop();
    // Hang if the main loop returns
    loop {}
}

// Bind a register's value to an identifier
macro_rules! reg {
    (let $var:ident = $reg:tt) => {
        reg!(let $var: u32 = $reg);
    };
    (let $var:ident: $size:ty = $reg:tt) => {
        let $var: $size;
        unsafe {
            asm! { "", out($reg) $var }
        }
    };
}

macro_rules! with_caller_saved_regs {
    ($($body:tt)*) => {
        let r2: u32;
        let r3: u32;
        let r4: u32;
        let r5: u32;
        let r6: u32;
        let r7: u32;
        let r8: u32;
        let r9: u32;
        let r10: u32;
        let r11: u32;
        let r12: u32;
        let r13: u32;
        let r14: u32;
        let r15: u32;
        let r24: u32;
        let r25: u32;
        unsafe {
            asm! { "",
                out("$2") r2,
                out("$3") r3,
                out("$4") r4,
                out("$5") r5,
                out("$6") r6,
                out("$7") r7,
                out("$8") r8,
                out("$9") r9,
                out("$10") r10,
                out("$11") r11,
                out("$12") r12,
                out("$13") r13,
                out("$14") r14,
                out("$15") r15,
                out("$24") r24,
                out("$25") r25
            }
        }
        $($body)*
        unsafe {
            asm! { "",
                in("$2") r2,
                in("$3") r3,
                in("$4") r4,
                in("$5") r5,
                in("$6") r6,
                in("$7") r7,
                in("$8") r8,
                in("$9") r9,
                in("$10") r10,
                in("$11") r11,
                in("$12") r12,
                in("$13") r13,
                in("$14") r14,
                in("$15") r15,
                in("$24") r24,
                in("$25") r25
            }
        }
    };
}

// These are the four instructions that are written to the BIOS fn vectors
#[naked]
unsafe extern "C" fn fn_vec() {
    asm! {
        "la $10, fn_handler
         jr $10",
        options(noreturn)
    }
}

// The handler called by the three BIOS fn vectors.
#[no_mangle]
extern "C" fn fn_handler() {
    reg!(let fn_ty: u8 = "$8");
    reg!(let fn_num: u8 = "$9");
    if fn_num == STD_OUT_PUTCHAR_NUM && fn_ty == STD_OUT_PUTCHAR_TY {
        // Emulators usually implement debug output by checking that PC reaches
        // 0x8000_00B0 with $9 set to 0x3D so the BIOS just needs to return to the
        // caller in this case.
        return
    }
    println!("Called unimplemented function {:x}({:x})", fn_ty, fn_num);
}

#[naked]
unsafe extern "C" fn exception_vec() {
    asm! {
        ".set noreorder
         la $4, exception_handler
         jalr $4
         nop
         jr $26
         .long 0x42000010 #rfe
         .set reorder",
        options(noreturn)
    }
}

#[no_mangle]
unsafe extern "C" fn exception_handler() {
    with_caller_saved_regs! {
        let epc = cop0::EPC::new().to_bits();
        asm! {
            "move $26, $2", in("$2") epc
        }

        println!("Jumped to exception handler at {:#x?} because {:#x?}", epc, cop0::Cause::new());
        irq::Status::new().ack_all().store();
    };
}

#[no_mangle]
extern "C" fn init_vectors() {
    // Write the instructions above to the fn vectors
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

#[no_mangle]
extern "C" fn main_loop() {
    println!("Starting main BIOS loop");
    let mut sr = cop0::Status::new();
    sr.enable_interrupts()
        .unmask_interrupt(IntSrc::Hardware)
        .use_boot_vectors(false)
        .store();
    println!("{:#x?}", sr);
    let mut mask = irq::Mask::new();
    mask.enable_all().store();
}
