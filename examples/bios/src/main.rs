#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]
#![feature(naked_functions)]
#![feature(core_intrinsics)]

use core::arch::asm;
use core::ffi::CStr;
use psx::hw::cop0;
use psx::hw::cop0::IntSrc;
use psx::hw::irq;
use psx::hw::Register;
use psx::sys::kernel::*;

mod boot;
mod exceptions;
mod misc;
mod rand;
mod stdout;
mod thread;

use crate::misc::get_system_info;
use crate::rand::{rand, srand};
use crate::thread::{change_thread, close_thread, open_thread};

fn main() {
    println!("Starting main BIOS loop");
    println!("{:?}", psx::sys::get_system_version());
    println!("{:x?}", psx::sys::get_system_date());
    let mut sr = cop0::Status::new();
    sr.enable_interrupts()
        .unmask_interrupt(IntSrc::Hardware)
        .use_boot_vectors(false)
        .store();
    println!("{:#x?}", sr);
    let mut mask = irq::Mask::new();
    loop {
        mask.enable_all().store();
    }
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
extern "C" fn fn_handler() -> u32 {
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

    reg!(let fn_ty: u8 = "$8");
    reg!(let fn_num: u8 = "$9");
    // TODO: Consider switching to the table of function pointers approached
    // used by other BIOS implementations
    match (fn_num, fn_ty) {
        (SRAND_NUM, SRAND_TY) => {
            reg!(let seed = "$4");
            srand(seed);
            0
        },
        (RAND_NUM, RAND_TY) => rand(),
        (PRINTF_NUM, PRINTF_TY) => {
            reg!(let fmt_str = "$4");
            reg!(let arg0 = "$5");
            reg!(let arg1 = "$6");
            reg!(let arg2 = "$7");
            let args = [arg0, arg1, arg2];
            // SAFETY: Let's hope the user passed in a null-terminated string
            let fmt_str = unsafe { CStr::from_ptr(fmt_str as *const i8) };
            let mut va_arg = None;
            let mut args_used = 0;
            for &b in fmt_str.to_bytes() {
                if b == b'%' {
                    if va_arg.is_none() {
                        va_arg = Some(args_used);
                        continue
                    } else {
                        va_arg = None;
                    }
                }
                match va_arg {
                    Some(idx) => {
                        match b {
                            b'd' | b'i' | b'D' => {
                                print!("{}", args[idx]);
                                args_used += 1;
                            },
                            b'x' => {
                                print!("{:x}", args[idx]);
                                args_used += 1;
                            },
                            b'X' => {
                                print!("{:X}", args[idx]);
                                args_used += 1;
                            },
                            b's' => {
                                // SAFETY: Let's hope the user passed in a null-terminated string
                                let str_arg = unsafe { CStr::from_ptr(args[idx] as *const i8) };
                                print!("{}", str_arg.to_str().unwrap());
                                args_used += 1;
                            },
                            _ => {},
                        }
                        va_arg = None;
                    },
                    None => unsafe {
                        psx_std_out_putchar(b);
                    },
                }
            }
            0
        },
        (GET_SYSTEM_INFO_NUM, GET_SYSTEM_INFO_TY) => {
            reg!(let idx: u8 = "$4");
            get_system_info(idx)
        },
        (OPEN_THREAD_NUM, OPEN_THREAD_TY) => {
            reg!(let pc: u32 = "$4");
            reg!(let sp: u32 = "$5");
            reg!(let gp: u32 = "$6");
            open_thread(pc, sp, gp)
        },
        (CHANGE_THREAD_NUM, CHANGE_THREAD_TY) => {
            reg!(let handle: u32 = "$4");
            change_thread(handle)
        },
        (CLOSE_THREAD_NUM, CLOSE_THREAD_TY) => {
            reg!(let handle: u32 = "$4");
            close_thread(handle)
        },
        (STD_OUT_PUTCHAR_NUM, STD_OUT_PUTCHAR_TY) => {
            // Emulators usually implement debug output by checking that PC reaches
            // 0x8000_00B0 with $9 set to 0x3D so the BIOS just needs to return to the
            // caller in this case.
            0
        },
        _ => {
            println!("Called unimplemented function {:x}({:x})", fn_ty, fn_num);
            u32::MAX
        },
    }
}
