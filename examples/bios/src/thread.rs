extern crate alloc;
use crate::global::Global;
use alloc::boxed::Box;
use alloc::vec;
use core::ffi::CStr;
use core::fmt;
use core::fmt::{Debug, Formatter};
use core::mem::{size_of, transmute};
use core::ptr;
use psx::constants::KB;
use psx::hw::{cop0, Register};
use psx::sys::kernel::psx_change_thread_sub_fn;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ThreadHandle(pub u32);

const MAIN_THREAD: ThreadHandle = ThreadHandle(0xFF00_0000);
const INVALID_HANDLE: ThreadHandle = ThreadHandle(0xFFFF_FFFF);

#[derive(Debug)]
pub struct Thread<'a> {
    handle: ThreadHandle,
    // The thread's stack
    stack: &'a mut [u32],
}

impl<'a> Thread<'a> {
    pub fn new(entry_point: extern "C" fn() -> !) -> Option<Self> {
        let func = unsafe { transmute(entry_point) };
        Self::new_with_args(func, [0; 4])
    }

    pub fn new_with_args(
        entry_point: extern "C" fn(u32, u32, u32, u32) -> !, args: [u32; 4],
    ) -> Option<Self> {
        let default_stack_size = KB / size_of::<u32>();
        Self::new_with_stack(entry_point, args, default_stack_size)
    }

    pub fn new_with_stack(
        entry_point: extern "C" fn(u32, u32, u32, u32) -> !, args: [u32; 4], stack_size: usize,
    ) -> Option<Self> {
        let mut stack = vec![0u32; stack_size].into_boxed_slice();
        let handle = open_thread(
            entry_point as *const u32,
            &mut stack[stack_size - 1],
            ptr::null_mut(),
            args,
        );
        // Leak the stack to avoid freeing it if the Thread is dropped
        let stack = Box::leak(stack);
        if handle == INVALID_HANDLE {
            None
        } else {
            Some(Self { handle, stack })
        }
    }

    pub fn resume(&mut self) {
        change_thread(self.handle);
    }

    pub fn resume_main() {
        change_thread(MAIN_THREAD);
    }

    pub fn close(self) {
        close_thread(self.handle);
        // If the Thread is manually closed, free its stack memory
        let stack = unsafe { Box::from_raw(self.stack) };
        drop(stack);
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct ThreadControlBlock {
    // General purpose registers except r0
    pub regs: [u32; 31],
    // The mul/div registers
    pub mul_div_regs: [u32; 2],
    // cop0 r12, r13 and r14
    pub cop0_regs: [u32; 3],
}

impl Debug for ThreadControlBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut dbg_s = f.debug_struct("ThreadControlBlock");

        let mut reg_name_arr = [0; 4];
        reg_name_arr[0] = b'R';
        for (n, &gpr) in self.regs.iter().enumerate() {
            let n = n as u8;
            if n < 10 {
                reg_name_arr[1] = n + b'0';
            } else if n < 20 {
                reg_name_arr[1] = b'1';
                reg_name_arr[2] = n - 10 + b'0';
            } else if n < 30 {
                reg_name_arr[1] = b'2';
                reg_name_arr[2] = n - 20 + b'0';
            } else {
                reg_name_arr[1] = b'3';
                reg_name_arr[2] = n - 30 + b'0';
            }
            let reg_name_cstr = unsafe { CStr::from_ptr(reg_name_arr.as_ptr().cast()) };
            let reg_name = reg_name_cstr.to_str().unwrap();
            dbg_s.field(reg_name, &gpr);
        }
        dbg_s
            .field("lo", &self.mul_div_regs[0])
            .field("hi", &self.mul_div_regs[1])
            .field("sr", &self.cop0_regs[0])
            .field("cause", &self.cop0_regs[1])
            .field("epc", &self.cop0_regs[2])
            .finish()
    }
}

impl ThreadControlBlock {
    pub const fn new() -> Self {
        Self {
            regs: [0; 31],
            mul_div_regs: [0; 2],
            cop0_regs: [0; 3],
        }
    }

    pub fn cop0_cause(&mut self) -> &mut u32 {
        &mut self.cop0_regs[1]
    }
    pub fn cop0_epc(&mut self) -> &mut u32 {
        &mut self.cop0_regs[2]
    }
}

const NUM_THREADS: usize = 4;

#[no_mangle]
static THREADS: Global<[ThreadControlBlock; NUM_THREADS]> =
    Global::new([const { ThreadControlBlock::new() }; NUM_THREADS]);

static IN_USE: Global<[bool; NUM_THREADS]> = Global::new([true, false, false, false]);

#[no_mangle]
static CURRENT_THREAD: Global<u16> = Global::new(0);

pub unsafe fn get_current_thread<'a>() -> &'a mut ThreadControlBlock {
    &mut THREADS.as_ref()[*CURRENT_THREAD.as_ref() as usize]
}

pub unsafe fn set_current_thread(idx: u32) {
    *CURRENT_THREAD.as_ref() = idx as u16;
}

pub fn open_thread(pc: *const u32, sp: *mut u32, gp: *mut u32, args: [u32; 4]) -> ThreadHandle {
    cop0::Status::new().critical_section(|| {
        let threads = unsafe { THREADS.as_ref() };
        for (i, t) in threads.iter_mut().enumerate() {
            let in_use = unsafe { &mut IN_USE.as_ref()[i] };
            if !*in_use {
                let mut regs = [0; 31];
                let mut cop0_regs = [0; 3];
                regs[3] = args[0];
                regs[4] = args[1];
                regs[5] = args[2];
                regs[6] = args[3];
                // r0 is not included, so indices are off by 1
                regs[27] = gp as u32;
                regs[28] = sp as u32;
                // This is the frame pointer
                regs[29] = sp as u32;
                // This is the program counter after returning from an exception
                cop0_regs[2] = pc as u32;
                *t = ThreadControlBlock {
                    regs,
                    mul_div_regs: [0; 2],
                    cop0_regs,
                };
                *in_use = true;
                return ThreadHandle(MAIN_THREAD.0 | (i as u32))
            }
        }
        INVALID_HANDLE
    })
}

fn handle_to_idx(handle: u32) -> Option<usize> {
    match handle {
        0xFF00_0000 => Some(0),
        0xFF00_0001 => Some(1),
        0xFF00_0002 => Some(2),
        0xFF00_0003 => Some(3),
        _ => None,
    }
}

pub fn change_thread(handle: ThreadHandle) -> u32 {
    let new = match handle_to_idx(handle.0) {
        Some(idx) => idx,
        None => return 1,
    };
    if unsafe { IN_USE.as_ref()[new] } {
        unsafe { psx_change_thread_sub_fn(0, new) }
    };
    1
}

pub fn close_thread(handle: ThreadHandle) -> u32 {
    let idx = match handle_to_idx(handle.0) {
        Some(idx) => idx,
        None => return 1,
    };
    cop0::Status::new().critical_section(|| unsafe {
        IN_USE.as_ref()[idx] = false;
    });
    1
}
