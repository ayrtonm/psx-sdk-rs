#![allow(dead_code)]

extern crate alloc;
use crate::global::Global;
use alloc::boxed::Box;
use alloc::vec;
use bytemuck::bytes_of_mut;
use core::ffi::CStr;
use core::fmt;
use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;
use core::mem::{size_of, transmute};
use core::ptr;
use core::slice;
use psx::constants::KB;
use psx::hw::{cop0, Register};
use psx::sys::kernel::psx_change_thread_sub_fn;
use psx::CriticalSection;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ThreadHandle(pub usize);

const MAIN_THREAD: usize = 0xFF00_0000;

impl ThreadHandle {
    fn new(idx: usize) -> Self {
        Self(MAIN_THREAD | idx)
    }
    fn invalid() -> Self {
        Self(0xFFFF_FFFF)
    }
    fn get_idx(&self) -> usize {
        self.0 & !MAIN_THREAD
    }
    fn is_invalid(&self) -> bool {
        *self == ThreadHandle::invalid()
    }
}

// SAFETY: The pointer in ThreadStack came from a &mut [u32] and is only used in
// Thread which does not implement Copy or Clone so there can ever only be one
// copy of it.
unsafe impl Send for ThreadStack {}

#[repr(C)]
#[derive(Debug)]
struct ThreadStack {
    stack_ptr: *mut u32,
    stack_len: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct Thread<A: Send, R: Send> {
    handle: ThreadHandle,
    // We need to track the thread's stack to free it when the thread is manually closed. Ideally
    // this would be a &mut [u32], but slices are not FFI-safe so we need to deconstruct the slice
    // to allow `impl Send for Thread`.
    stack: ThreadStack,
    _arg: PhantomData<A>,
    _ret: PhantomData<R>,
}

extern "C" fn resume<const N: usize>() {
    change_thread(ThreadHandle::new(N), false);
}

pub fn resume_main() {
    resume::<0>()
}

pub fn park() {
    cop0::Status::new().critical_section(|cs| {
        let threads = THREADS.borrow(cs);
        let idx = threads.iter().position(|tcb| tcb.running).unwrap();
        let mut current_tcb = &mut threads[idx];
        current_tcb.parked = true;
    });
}

impl<R: Send> Thread<(), R> {
    const RET_SIZE: () = {
        if size_of::<R>() > size_of::<u64>() {
            panic!("Thread return type is too large");
        }
    };

    #[allow(path_statements)]
    pub fn create(entry_point: extern "C" fn() -> R) -> Option<Self> {
        Self::RET_SIZE;
        // SAFETY: create_with_stack ensures that the TCB's argument registers are
        // initialized with the argument/zero-initialized as necessary
        let entry_with_arg = unsafe { transmute(entry_point) };
        Self::create_with_arg(entry_with_arg, ())
    }
}

impl<A: Send, R: Send> Thread<A, R> {
    const ARG_SIZE: () = {
        if size_of::<A>() > size_of::<u128>() {
            panic!("Thread argument type is too large");
        }
    };
    #[allow(path_statements)]
    pub fn create_with_arg(entry_point: extern "C" fn(A) -> R, arg: A) -> Option<Self> {
        Self::ARG_SIZE;
        let default_stack_size = 2 * KB / size_of::<u32>();
        Self::create_with_stack(entry_point, arg, default_stack_size)
    }

    pub fn create_with_stack(
        entry_point: extern "C" fn(A) -> R, arg: A, stack_size: usize,
    ) -> Option<Self> {
        let mut stack = vec![0u32; stack_size].into_boxed_slice();
        let stack_end = stack.as_mut_ptr();

        // arg may be smaller than 128 bits so we can't just cast it's address and read
        // it instead we have to copy it to a 128 bit array then read that
        let mut tmp_arg = [0u32; 4];
        if size_of::<A>() > 0 {
            // arg might not be 4-byte aligned so we have to copy byte by byte
            // SAFETY: A has to be FFI-safe so it should be fine to cast to a byte slice
            let arg_as_bytes =
                unsafe { slice::from_raw_parts(&arg as *const A as *const u8, size_of::<A>()) };
            let tmp_as_bytes = bytes_of_mut(&mut tmp_arg);
            tmp_as_bytes[0..arg_as_bytes.len()].copy_from_slice(arg_as_bytes);
        }

        let handle = open_thread(
            entry_point as *const u32,
            &mut stack[stack_size - 1],
            ptr::null_mut(),
            tmp_arg,
            stack_end,
        );
        // Leak the stack to avoid freeing it if the Thread is dropped
        let stack = Box::leak(stack);
        if handle.is_invalid() {
            None
        } else {
            Some(Self {
                handle,
                stack: ThreadStack {
                    stack_ptr: stack.as_mut_ptr(),
                    stack_len: stack.len(),
                },
                _arg: PhantomData,
                _ret: PhantomData,
            })
        }
    }

    pub fn unpark(&mut self) {
        cop0::Status::new().critical_section(|cs| {
            let tcb = &mut THREADS.borrow(cs)[self.handle.get_idx()];
            tcb.parked = false;
        });
    }

    pub fn join(mut self) -> R {
        self.resume();
        let regs = cop0::Status::new().critical_section(|cs| {
            // SAFETY: static mut access in a critical section
            let threads = THREADS.borrow(cs);
            let tcb = &threads[self.handle.get_idx()];
            let v0 = tcb.regs[1] as u64;
            let v1 = tcb.regs[2] as u64;
            if size_of::<R>() > size_of::<u32>() {
                v0 | (v1 << 32)
            } else {
                v0
            }
        });
        let ptr = &regs as *const u64 as *const R;
        // SAFETY: The thread function ensures that the return register(s) has a valid
        // value for R. The value at ptr may not be used since the thread has returned,
        // so this cannot violate memory safety even if R is not Copy.
        let res = unsafe { ptr.read_unaligned() };
        self.close();
        res
    }

    pub fn resume(&mut self) {
        change_thread(self.handle, true);
    }

    pub fn close(self) {
        // Mark the TCB as not in use
        close_thread(self.handle);
        // If the Thread is manually closed, free its stack memory
        // SAFETY: This pointer and length came from the slice we got from leaking the
        // stack when opening the thread. Since the thread has returned at this point,
        // it's no longer using the stack.
        let stack_slice =
            unsafe { slice::from_raw_parts_mut(self.stack.stack_ptr, self.stack.stack_len) };
        // SAFETY: This came from the leaked Box above and close consumes self, so we
        // can't free this twice
        let stack = unsafe { Box::from_raw(stack_slice) };
        drop(stack);
    }
}

#[repr(C)]
pub struct ThreadControlBlock {
    // The three register fields are accessed from asm so their order and sizes matters
    // General purpose registers except r0
    pub regs: [u32; 31],
    // The mul/div registers
    pub mul_div_regs: [u32; 2],
    // cop0 r12, r13 and r14
    pub cop0_regs: [u32; 3],

    in_use: bool,
    running: bool,
    parked: bool,
    stack: *mut u32,
    time_remaining: u8,
    allocated_time: u8,
}

impl Debug for ThreadControlBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut dbg_s = f.debug_struct("ThreadControlBlock");

        let mut reg_name_arr = [0; 4];
        reg_name_arr[0] = b'R';
        dbg_s.field("R0", &0);
        for (n, &gpr) in self.regs.iter().enumerate() {
            let n = n as u8 + 1;
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
            // SAFETY: reg_name_arr has a null terminator at the end
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
            .field("in_use", &self.in_use)
            .field("running", &self.running)
            .field("parked", &self.parked)
            .finish()
    }
}

impl ThreadControlBlock {
    pub const fn new(regs: [u32; 31], cop0_regs: [u32; 3]) -> Self {
        Self {
            regs,
            mul_div_regs: [0; 2],
            cop0_regs,
            in_use: false,
            running: false,
            parked: true,
            stack: ptr::null_mut(),
            time_remaining: 1,
            allocated_time: 1,
        }
    }

    pub fn sp(&self) -> u32 {
        self.regs[28]
    }

    pub fn cop0_cause(&mut self) -> &mut u32 {
        &mut self.cop0_regs[1]
    }
    pub fn cop0_epc(&mut self) -> &mut u32 {
        &mut self.cop0_regs[2]
    }
}

#[inline(always)]
pub fn reschedule_threads(tcb: *mut ThreadControlBlock, cs: &mut CriticalSection) -> bool {
    unsafe {
        (*tcb).time_remaining -= 1;
        if (*tcb).time_remaining != 0 {
            return false
        }
        (*tcb).time_remaining = (*tcb).allocated_time;
    }

    let threads = THREADS.borrow(cs);
    let unparked = |tcb: &&mut ThreadControlBlock| tcb.in_use && !tcb.parked;

    // If there's only one unparked thread then there's no scheduling to do
    if threads.iter_mut().filter(unparked).count() == 1 {
        return false
    }

    let mut next_thread = 0;
    let mut set_next_thread = false;
    for (i, tcb) in threads.iter_mut().filter(unparked).enumerate() {
        if set_next_thread {
            next_thread = i;
            // If we found the next TCB, then we're done iterating
            break
        }
        if tcb.running {
            // Set the next TCB as the next thread. If we're at the end of the iterator, the
            // next TCB will be unparked TCB 0 since we initialized next_thread above.
            set_next_thread = true;
            // Mark the running TCB as not running
            tcb.running = false;
        }
    }
    unsafe {
        let next_tcb = threads
            .iter_mut()
            .filter(unparked)
            .nth(next_thread)
            .unwrap_unchecked();
        // Mark the next TCB as running
        next_tcb.running = true;
        *CURRENT_THREAD.borrow(cs) = next_tcb;
    }
    true
}

#[cold]
pub fn init_threads(cs: &mut CriticalSection) {
    *CURRENT_THREAD.borrow(cs) = THREADS.borrow(cs).as_mut_ptr();
}

#[no_mangle]
pub static CURRENT_THREAD: Global<*mut ThreadControlBlock> = Global::new(ptr::null_mut());

static THREADS: Global<[ThreadControlBlock; 4]> = {
    let mut tcbs = [const { ThreadControlBlock::new([0; 31], [0; 3]) }; 4];
    tcbs[0].in_use = true;
    tcbs[0].running = true;
    tcbs[0].parked = false;
    tcbs[0].allocated_time = 128;
    tcbs[0].time_remaining = tcbs[0].allocated_time;
    Global::new(tcbs)
};

pub fn open_thread(
    pc: *const u32, sp: *mut u32, gp: *mut u32, args: [u32; 4], stack: *mut u32,
) -> ThreadHandle {
    let mut sr = cop0::Status::new();
    let old_sr = sr.to_bits();
    let old_cause = cop0::Cause::new().to_bits();
    sr.critical_section(|cs| {
        let threads = THREADS.borrow(cs);
        for (i, tcb) in threads.iter_mut().enumerate() {
            if !tcb.in_use {
                let mut regs = [0; 31];
                let mut cop0_regs = [0; 3];
                // r0 is not included, so indices are off by 1
                regs[3] = args[0];
                regs[4] = args[1];
                regs[5] = args[2];
                regs[6] = args[3];
                regs[27] = gp as u32;
                regs[28] = sp as u32;
                // This is the frame pointer
                regs[29] = sp as u32;
                cop0_regs[0] = cop0::Status::from_bits(old_sr)
                    .previous_interrupt_enable()
                    .disable_interrupts()
                    .to_bits();
                cop0_regs[1] = old_cause;
                // This is the program counter after returning from an exception
                cop0_regs[2] = pc as u32;
                *tcb = ThreadControlBlock::new(regs, cop0_regs);
                tcb.in_use = true;
                tcb.stack = stack;
                return ThreadHandle::new(i)
            }
        }
        ThreadHandle::invalid()
    })
}

pub fn change_thread(handle: ThreadHandle, set_ra: bool) -> u32 {
    let new = handle.get_idx();
    cop0::Status::new().critical_section(|cs| {
        let threads = THREADS.borrow(cs);
        if threads[new].in_use {
            let old = threads.iter().position(|tcb| tcb.running).unwrap();
            threads[old].running = false;
            threads[new].running = true;
            threads[new].parked = false;
            if set_ra {
                let resume_fns = [resume::<0>, resume::<1>, resume::<2>, resume::<3>];
                let ra = resume_fns[old];
                threads[new].regs[30] = ra as u32;
            }
            let tcb_ptr = &threads[new] as *const ThreadControlBlock;
            let unused_arg = 0;
            // SAFETY: The second argument is a TCB pointer
            unsafe { psx_change_thread_sub_fn(unused_arg, tcb_ptr as usize) }
        };
    });
    1
}

pub fn close_thread(handle: ThreadHandle) -> u32 {
    let idx = handle.get_idx();
    cop0::Status::new().critical_section(|cs| {
        let mut thread = &mut THREADS.borrow(cs)[idx];
        thread.in_use = false;
        thread.running = false;
        thread.parked = true;
    });
    1
}
