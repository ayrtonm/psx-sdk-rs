use crate::global::Global;
use core::arch::asm;
use psx::hw::{cop0, Register};

#[derive(Debug, Clone, Copy)]
struct ThreadControlBlock {
    // General purpose registers except r0
    regs: [u32; 31],
    // The mul/div registers
    mul_div_regs: [u32; 2],
    // cop0 r12, r13 and r14
    cop0_regs: [u32; 3],
}

static THREADS: Global<[Option<ThreadControlBlock>; 4]> = Global::new([
    Some(ThreadControlBlock {
        regs: [0; 31],
        mul_div_regs: [0; 2],
        cop0_regs: [0; 3],
    }),
    None,
    None,
    None,
]);

static CURRENT_THREAD: Global<usize> = Global::new(0);

pub fn open_thread(pc: u32, sp: u32, gp: u32) -> u32 {
    let mut sr = cop0::Status::new();
    THREADS.ensure_mut(&mut sr, |threads, _| {
        for (i, t) in threads.iter_mut().enumerate() {
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
                    mul_div_regs: [0; 2],
                    cop0_regs,
                });
                return 0xFF00_0000 | (i as u32)
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
    let mut sr = cop0::Status::new();
    let tcbs = THREADS.ensure_mut(&mut sr, |threads, _| {
        let current = unsafe { CURRENT_THREAD.assume_mut() };
        let current_tcb = threads[*current].expect("CURRENT_THREAD marked as not in use");
        let new_tcb = match threads[new] {
            Some(tcb) => tcb,
            None => return None,
        };
        let res = (current_tcb, new_tcb);
        *current = new;
        Some(res)
    });
    let (old_tcb, new_tcb) = match tcbs {
        Some((old, new)) => (old, new),
        None => return 1,
    };
    unsafe {
        asm! {
            "",
                out("$2") old_tcb.regs[1],
                out("$3") old_tcb.regs[2],
                out("$4") old_tcb.regs[3],
                out("$5") old_tcb.regs[4],
                out("$6") old_tcb.regs[5],
                out("$7") old_tcb.regs[6],
                out("$8") old_tcb.regs[7],
                out("$9") old_tcb.regs[8],
                out("$10") old_tcb.regs[9],
                out("$11") old_tcb.regs[10],
                out("$12") old_tcb.regs[11],
                out("$13") old_tcb.regs[12],
                out("$14") old_tcb.regs[13],
                out("$15") old_tcb.regs[14],
                out("$16") old_tcb.regs[15],
                out("$17") old_tcb.regs[16],
                out("$18") old_tcb.regs[17],
                out("$19") old_tcb.regs[18],
                out("$20") old_tcb.regs[19],
                out("$21") old_tcb.regs[20],
                out("$22") old_tcb.regs[21],
                out("$23") old_tcb.regs[22],
                out("$24") old_tcb.regs[23],
                out("$25") old_tcb.regs[24],
        }
        asm! {
            "",
                in("$2") new_tcb.regs[1],
                in("$3") new_tcb.regs[2],
                in("$4") new_tcb.regs[3],
                in("$5") new_tcb.regs[4],
                in("$6") new_tcb.regs[5],
                in("$7") new_tcb.regs[6],
                in("$8") new_tcb.regs[7],
                in("$9") new_tcb.regs[8],
                in("$10") new_tcb.regs[9],
                in("$11") new_tcb.regs[10],
                in("$12") new_tcb.regs[11],
                in("$13") new_tcb.regs[12],
                in("$14") new_tcb.regs[13],
                in("$15") new_tcb.regs[14],
                in("$16") new_tcb.regs[15],
                in("$17") new_tcb.regs[16],
                in("$18") new_tcb.regs[17],
                in("$19") new_tcb.regs[18],
                in("$20") new_tcb.regs[19],
                in("$21") new_tcb.regs[20],
                in("$22") new_tcb.regs[21],
                in("$23") new_tcb.regs[22],
                in("$24") new_tcb.regs[23],
                in("$25") new_tcb.regs[24],
                //in("$26") new_tcb.regs[25],
                //in("$27") new_tcb.regs[26],
                //in("$28") new_tcb.regs[27],
                //in("$29") new_tcb.regs[28],
                //in("$30") new_tcb.regs[29],
                //in("$31") new_tcb.regs[30],
                options(noreturn)
        }
    }
}

pub fn close_thread(handle: u32) -> u32 {
    let idx = match handle_to_idx(handle) {
        Some(idx) => idx,
        None => return 1,
    };
    let mut sr = cop0::Status::new();
    THREADS.ensure_mut(&mut sr, |threads, _| threads[idx] = None);
    1
}
