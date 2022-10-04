#[derive(Debug, Clone, Copy)]
struct ThreadControlBlock {
    // General purpose registers except r0
    regs: [u32; 31],
    // The mul/div registers
    mul_regs: [u32; 2],
    // cop0 r12, r13 and r14
    cop0_regs: [u32; 3],
}

static mut THREADS: [Option<ThreadControlBlock>; 4] = [None; 4];

pub fn open_thread(pc: u32, sp: u32, gp: u32) -> u32 {
    for (i, t) in unsafe { THREADS.iter_mut().enumerate() } {
        if t.is_none() {
            let mut regs = [0; 31];
            let mut cop0_regs = [0; 3];
            // r0 is not included, so indices are off by 1
            regs[27] = gp;
            regs[28] = sp;
            // This is the frame pointer
            regs[29] = sp;
            // This is the pc when returning from an exception
            cop0_regs[2] = pc;
            *t = Some(ThreadControlBlock {
                regs,
                mul_regs: [0; 2],
                cop0_regs,
            });
            return 0xFF00_0000 | (i as u32)
        }
    }
    0xFFFF_FFFF
}

pub fn change_thread(handle: u32) -> u32 {
    todo!("Implement change_thread");
    1
}

pub fn close_thread(handle: u32) -> u32 {
    let idx = match handle {
        0xFF00_0000 => 0,
        0xFF00_0001 => 1,
        0xFF00_0002 => 2,
        0xFF00_0003 => 3,
        _ => return 1,
    };
    unsafe {
        THREADS[idx] = None;
    }
    1
}
