#![allow(dead_code)]

extern crate alloc;
use crate::global::Global;
use crate::println;
use crate::thread::{ThreadControlBlock, CURRENT_THREAD};
use alloc::boxed::Box;
use core::arch::asm;
use core::ptr;
use core::ptr::NonNull;
use psx::hw::cop0;
use psx::hw::cop0::{Excode, IntSrc};
use psx::hw::irq;
use psx::hw::Register;
use psx::irq::IRQ;
use psx::sys::kernel::*;
use psx::CriticalSection;

#[naked]
pub unsafe extern "C" fn exception_vec() {
    asm! {
        ".set noreorder
         .set noat
         la $k0, exception_handler
         jr $k0
         nop
         .set at
         .set reorder",
        options(noreturn)
    }
}

#[naked]
#[no_mangle]
pub unsafe extern "C" fn exception_handler() {
    asm! {
        ".set noreorder
         .set noat
         la $k0, CURRENT_THREAD
         lw $k0, ($k0)
         mfc0 $k1, $14

         sw $at, ($k0)

         sw $v0, 4($k0)
         sw $v1, 8($k0)

         sw $a0, 12($k0)
         sw $a1, 16($k0)
         sw $a2, 20($k0)
         sw $a3, 24($k0)

         sw $t0, 28($k0)
         sw $t1, 32($k0)
         sw $t2, 36($k0)
         sw $t3, 40($k0)
         sw $t4, 44($k0)
         sw $t5, 48($k0)
         sw $t6, 52($k0)
         sw $t7, 56($k0)

         sw $t8, 92($k0)
         sw $t9, 96($k0)

         sw $ra, 120($k0)

         mflo $t0
         mfhi $t1
         # Set call_handlers 3rd argument
         mfc0 $a2, $13
         sw $t0, 124($k0)
         sw $t1, 128($k0)
         sw $a2, 136($k0)

         # Set call_handlers 4th argument
         move $a3, $k0

         # call_handlers is in ROM so we need jalr
         jal call_handlers
         nop

         # Check if there's a new TCB
         beqz $v0, 2f
         nop

         sw $s0, 60($k0)
         sw $s1, 64($k0)
         sw $s2, 68($k0)
         sw $s3, 72($k0)
         sw $s4, 76($k0)
         sw $s5, 80($k0)
         sw $s6, 84($k0)
         sw $s7, 88($k0)
         sw $sp, 112($k0)
         sw $fp, 116($k0)
         mfc0 $t2, $12
         sw $k1, 140($k0)
         sw $t2, 132($k0)

         lw $s0, 60($v0)
         lw $s1, 64($v0)
         lw $s2, 68($v0)
         lw $s3, 72($v0)
         lw $s4, 76($v0)
         lw $s5, 80($v0)
         lw $s6, 84($v0)
         lw $s7, 88($v0)
         lw $sp, 112($v0)
         lw $fp, 116($v0)
         lw $k1, 140($v0)
         lw $t2, 132($k0)
         move $k0, $v0
         mtc0 $t2, $12

         2:
         lw $at, ($k0)

         lw $t0, 124($k0)
         lw $t1, 128($k0)
         lw $t3, 136($k0)
         mtlo $t0
         mthi $t1
         mtc0 $t3, $13

         lw $v0, 4($k0)
         lw $v1, 8($k0)

         lw $a0, 12($k0)
         lw $a1, 16($k0)
         lw $a2, 20($k0)
         lw $a3, 24($k0)

         lw $t0, 28($k0)
         lw $t1, 32($k0)
         lw $t2, 36($k0)
         lw $t3, 40($k0)
         lw $t4, 44($k0)
         lw $t5, 48($k0)
         lw $t6, 52($k0)
         lw $t7, 56($k0)

         lw $t8, 92($k0)
         lw $t9, 96($k0)

         lw $ra, 120($k0)

         jr $k1
         .long 0x42000010 #rfe
         .set at
         .set reorder",
         options(noreturn)
    }
}

#[no_mangle]
#[inline(always)]
extern "C" fn call_handlers(
    r4: u32, r5: u32, cause: cop0::Cause, tcb: *mut ThreadControlBlock,
) -> *mut ThreadControlBlock {
    let mut cs = unsafe { CriticalSection::new() };
    let cs = &mut cs;
    let new_tcb = match cause.excode() {
        Excode::Interrupt => call_irq_handlers(tcb, cs),
        Excode::Syscall | Excode::Breakpoint => {
            unsafe {
                asm!("addiu $k1, 4");
            }
            if cause.excode() == Excode::Syscall {
                syscall_handler(cs, r4, r5)
            } else {
                println!("{:#x?}", tcb);
                ptr::null_mut()
            }
        },
        _ => unsafe { core::hint::unreachable_unchecked() },
    };
    new_tcb
}

pub struct IRQCtxt<'a> {
    pub tcb: *mut ThreadControlBlock,
    pub stat: &'a mut irq::Status,
    pub mask: &'a mut irq::Mask,
    pub active_irqs: irq::Requested,
    pub cs: &'a mut CriticalSection,
}

// It would've been nice to make this generic over the return type for handlers
// that don't switch threads, but the handler chain is a linked list so it
// would've either required `dyn IRQHandlerFn` or been all one type
pub type IRQHandlerFn = fn(IRQCtxt) -> *mut ThreadControlBlock;

pub struct IRQHandler {
    func: IRQHandlerFn,
    next: Option<Box<IRQHandler>>,
}

static HANDLER_CHAIN: Global<Option<IRQHandler>> = Global::new(None);

// This inserts the new handler at the end so it's really more of a stack...
pub fn enqueue_handler(func: IRQHandlerFn, cs: &mut CriticalSection) {
    let chain = HANDLER_CHAIN.borrow(cs);
    let handler = IRQHandler { func, next: None };
    match chain {
        None => *chain = Some(handler),
        Some(root) => {
            let mut next_handler = &mut root.next;
            while let Some(ref mut after_next) = next_handler {
                next_handler = &mut after_next.next;
            }
            *next_handler = Some(Box::new(handler));
        },
    }
}

pub fn dequeue_handler(cs: &mut CriticalSection) {
    let chain = HANDLER_CHAIN.borrow(cs);
    if let Some(root) = chain {
        match &mut root.next {
            None => *chain = None,
            Some(_) => {
                let mut next_handler = &mut root.next;
                while let Some(ref mut after_next) = next_handler {
                    next_handler = &mut after_next.next;
                }
                *next_handler = None;
            },
        }
    };
}

static AUTO_ACK: Global<irq::Requested> = Global::new(irq::Requested::new(0));

pub fn irq_auto_ack(irq: IRQ, auto_ack: bool, cs: &mut CriticalSection) {
    let irqs = AUTO_ACK.borrow(cs);
    if auto_ack {
        irqs.set(irq);
    } else {
        irqs.clear(irq);
    }
}

#[inline(always)]
fn call_irq_handlers(
    tcb: *mut ThreadControlBlock, cs: &mut CriticalSection,
) -> *mut ThreadControlBlock {
    let mut stat = irq::Status::new();
    let mut mask = irq::Mask::new();

    let mut new_tcb = ptr::null_mut();
    if let Some(root) = HANDLER_CHAIN.borrow(cs) {
        let active_irqs = mask.active_irqs(&stat);
        let ctxt = IRQCtxt {
            tcb,
            stat: &mut stat,
            mask: &mut mask,
            active_irqs,
            cs,
        };
        if let Some(tcb) = NonNull::new((root.func)(ctxt)) {
            new_tcb = tcb.as_ptr();
        };
        let mut next_handler = &root.next;
        while let Some(next) = next_handler {
            let ctxt = IRQCtxt {
                tcb: new_tcb,
                stat: &mut stat,
                mask: &mut mask,
                active_irqs,
                cs,
            };
            if let Some(tcb) = NonNull::new((next.func)(ctxt)) {
                new_tcb = tcb.as_ptr();
            };
            next_handler = &next.next;
        }
    };
    let mut auto_ack = false;
    for irq in AUTO_ACK.borrow(cs).iter() {
        stat.ack(irq);
        auto_ack = true;
    }
    // This volatile write only needs to happen if there was an auto ack
    if auto_ack {
        stat.store();
    }
    new_tcb
}

#[inline(always)]
fn syscall_handler(cs: &mut CriticalSection, r4: u32, r5: u32) -> *mut ThreadControlBlock {
    if r4 == ENTER_CRITICAL_SECTION_NUM as u32 {
        cop0::Status::new()
            .disable_interrupts()
            .mask_interrupt(IntSrc::Hardware)
            .store();
    } else if r4 == EXIT_CRITICAL_SECTION_NUM as u32 {
        cop0::Status::new()
            .enable_interrupts()
            .unmask_interrupt(IntSrc::Hardware)
            .store();
    } else if r4 == CHANGE_THREAD_SUB_FN_NUM as u32 {
        // SAFETY: This is safe to call in the exception handler
        let new_tcb = r5 as *mut ThreadControlBlock;
        *CURRENT_THREAD.borrow(cs) = new_tcb;
        return new_tcb
    } else {
        unsafe { core::hint::unreachable_unchecked() }
    }
    ptr::null_mut()
}
