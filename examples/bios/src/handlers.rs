use crate::allocator::{free, init_heap, malloc};
use crate::gamepad::init_pad;
use crate::misc::get_system_info;
use crate::println;
use crate::rand::{rand, srand};
use crate::stdout::printf;
use crate::thread::{change_thread, close_thread, open_thread, ThreadHandle};
use core::arch::{asm, naked_asm};
use core::ffi::CStr;
use core::mem::size_of;
use core::ptr;
use core::slice;
use psx::sys::kernel::*;

// These sets of four instructions are written to the BIOS fn vectors
#[naked]
pub unsafe extern "C" fn a0_fn_vec() {
    naked_asm! {
        ".set noreorder
         la $10, fn_handler
         jr $10
         or $8, $0, 0xA0
         .set reorder"
    }
}
#[naked]
pub unsafe extern "C" fn b0_fn_vec() {
    naked_asm! {
        ".set noreorder
         la $10, fn_handler
         jr $10
         or $8, $0, 0xB0
         .set reorder"
    }
}
#[naked]
pub unsafe extern "C" fn c0_fn_vec() {
    naked_asm! {
        ".set noreorder
         la $10, fn_handler
         jr $10
         or $8, $0, 0xC0
         .set reorder"
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
                asm! { "", out($reg) $var, options(nomem, nostack) }
            }
        };
    }

    reg!(let fn_ty: u8 = "$8");
    reg!(let fn_num: u8 = "$9");
    // TODO: Consider switching to the table of function pointers approached
    // used by other BIOS implementations
    match (fn_num, fn_ty) {
        (INIT_HEAP_NUM, INIT_HEAP_TY) => {
            reg!(let addr: *mut u8 = "$4");
            reg!(let len: usize = "$5");
            init_heap(addr, len)
        },
        (MALLOC_NUM, MALLOC_TY) => {
            reg!(let len: usize = "$4");
            malloc(len) as u32
        },
        (FREE_NUM, FREE_TY) => {
            reg!(let ptr: *mut u8 = "$4");
            free(ptr)
        },
        (SRAND_NUM, SRAND_TY) => {
            reg!(let seed = "$4");
            srand(seed)
        },
        (RAND_NUM, RAND_TY) => rand(),
        (PRINTF_NUM, PRINTF_TY) => {
            reg!(let fmt_ptr: *const i8 = "$4");
            reg!(let arg0 = "$5");
            reg!(let arg1 = "$6");
            reg!(let arg2 = "$7");
            // SAFETY: Let's hope the user passed in a null-terminated string
            let fmt_str = unsafe { CStr::from_ptr(fmt_ptr) };
            printf(fmt_str, arg0, arg1, arg2)
        },
        (GET_SYSTEM_INFO_NUM, GET_SYSTEM_INFO_TY) => {
            reg!(let idx: u8 = "$4");
            get_system_info(idx)
        },
        (OPEN_THREAD_NUM, OPEN_THREAD_TY) => {
            reg!(let pc: *const u32 = "$4");
            reg!(let sp: *mut u32 = "$5");
            reg!(let gp: *mut u32 = "$6");
            open_thread(pc, sp, gp, [0; 4], ptr::null_mut()).0 as u32
        },
        (CHANGE_THREAD_NUM, CHANGE_THREAD_TY) => {
            reg!(let handle: usize = "$4");
            change_thread(ThreadHandle(handle), false)
        },
        (CLOSE_THREAD_NUM, CLOSE_THREAD_TY) => {
            reg!(let handle: usize = "$4");
            close_thread(ThreadHandle(handle))
        },
        (INIT_PAD_NUM, INIT_PAD_TY) => {
            reg!(let ptr1: *mut u16 = "$4");
            reg!(let len1: usize = "$5");
            reg!(let ptr2: *mut u16 = "$6");
            reg!(let len2: usize = "$7");
            let buf1 = unsafe { slice::from_raw_parts_mut(ptr1, len1 / size_of::<u16>()) };
            let buf2 = unsafe { slice::from_raw_parts_mut(ptr2, len2 / size_of::<u16>()) };
            init_pad(buf1, buf2)
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
