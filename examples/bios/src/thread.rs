use crate::global::Global;
use core::fmt;
use core::fmt::{Debug, Formatter};
use core::mem::transmute;
use psx::hw::{cop0, Register};
use psx::sys::kernel::psx_change_thread_sub_fn;

pub const MAIN_THREAD: u32 = 0xFF00_0000;

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
        let mut reg_name = *b"R0 ";
        let mut ones_place = 1;
        let tens_place = 1;
        for gpr in self.regs {
            if reg_name[ones_place] == b'9' {
                if ones_place == 1 {
                    reg_name[tens_place] = b'1';
                    ones_place += 1;
                } else {
                    reg_name[tens_place] += 1;
                }
                reg_name[ones_place] = b'0';
            } else {
                reg_name[ones_place] += 1;
            }
            dbg_s.field(
                unsafe { transmute(reg_name[0..ones_place + 1].as_ref()) },
                &gpr,
            );
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
}

const NUM_THREADS: usize = 4;

#[no_mangle]
static THREADS: Global<[ThreadControlBlock; NUM_THREADS]> =
    Global::new([const { ThreadControlBlock::new() }; NUM_THREADS]);

static IN_USE: Global<[bool; NUM_THREADS]> = Global::new([true, false, false, false]);

#[no_mangle]
static CURRENT_THREAD: Global<usize> = Global::new(0);

pub unsafe fn get_current_thread<'a>() -> &'a mut ThreadControlBlock {
    &mut THREADS.assume_mut()[*CURRENT_THREAD.as_mut()]
}

pub unsafe fn set_current_thread(idx: u32) {
    *CURRENT_THREAD.as_mut() = idx as usize;
}

pub fn open_thread(pc: u32, sp: u32, gp: u32) -> u32 {
    let mut sr = cop0::Status::new();
    THREADS.ensure_mut(&mut sr, |threads, _| {
        for (i, t) in threads.iter_mut().enumerate() {
            let in_use = unsafe { &mut IN_USE.assume_mut()[i] };
            if !*in_use {
                let mut regs = [0; 31];
                let mut cop0_regs = [0; 3];
                // r0 is not included, so indices are off by 1
                regs[27] = gp;
                regs[28] = sp;
                // This is the frame pointer
                regs[29] = sp;
                // This is the program counter after returning from an exception
                cop0_regs[2] = pc;
                *t = ThreadControlBlock {
                    regs,
                    mul_div_regs: [0; 2],
                    cop0_regs,
                };
                *in_use = true;
                return MAIN_THREAD | (i as u32)
            }
        }
        0xFFFF_FFFF
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

pub fn change_thread(handle: u32) -> u32 {
    let new = match handle_to_idx(handle) {
        Some(idx) => idx,
        None => return 1,
    };
    if unsafe { IN_USE.assume_mut()[new] } {
        unsafe { psx_change_thread_sub_fn(0, new) }
    };
    1
}

pub fn close_thread(handle: u32) -> u32 {
    let idx = match handle_to_idx(handle) {
        Some(idx) => idx,
        None => return 1,
    };
    let mut sr = cop0::Status::new();
    IN_USE.ensure_mut(&mut sr, |t, _| t[idx] = false);
    1
}
