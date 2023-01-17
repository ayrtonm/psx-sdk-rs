use crate::println;
use crate::thread::{reschedule_threads, ThreadControlBlock, CURRENT_THREAD};
use core::arch::asm;
use core::ptr;
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
         nop

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
         mfc0 $k1, $14
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
        Excode::Interrupt => irq_handler(tcb, cs),
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

#[inline(always)]
fn irq_handler(tcb: *mut ThreadControlBlock, cs: &mut CriticalSection) -> *mut ThreadControlBlock {
    let mut stat = irq::Status::new();
    let mask = irq::Mask::new();

    let mut new_tcb = ptr::null_mut();
    for irq in mask.active_irqs(&stat) {
        if let Some(irq) = irq {
            match irq {
                IRQ::Vblank => {
                    new_tcb = vblank_handler(tcb, cs);
                },
                _ => {
                    println!("No handler installed for interrupt {irq:?}");
                },
            }
        }
    }
    stat.ack_all().store();
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

// A vblank handler repurposed to schedule threads
#[inline(always)]
fn vblank_handler(
    tcb: *mut ThreadControlBlock, cs: &mut CriticalSection,
) -> *mut ThreadControlBlock {
    reschedule_threads(tcb, cs)
}
