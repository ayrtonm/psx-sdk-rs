use crate::println;
use core::arch::asm;
use psx::hw::cop0;
use psx::hw::irq;
use psx::hw::Register;

macro_rules! with_caller_saved_regs {
    ($($body:tt)*) => {
        let r2: u32; let r3: u32; let r4: u32; let r5: u32;
        let r6: u32; let r7: u32; let r8: u32; let r9: u32;
        let r10: u32; let r11: u32; let r12: u32; let r13: u32;
        let r14: u32; let r15: u32; let r24: u32; let r25: u32;
        unsafe {
            asm! { "",
                out("$2") r2, out("$3") r3, out("$4") r4, out("$5") r5,
                out("$6") r6, out("$7") r7, out("$8") r8, out("$9") r9,
                out("$10") r10, out("$11") r11, out("$12") r12, out("$13") r13,
                out("$14") r14, out("$15") r15, out("$24") r24, out("$25") r25
            }
        }
        $($body)*
        unsafe {
            asm! { "",
                in("$2") r2, in("$3") r3, in("$4") r4, in("$5") r5,
                in("$6") r6, in("$7") r7, in("$8") r8, in("$9") r9,
                in("$10") r10, in("$11") r11, in("$12") r12, in("$13") r13,
                in("$14") r14, in("$15") r15, in("$24") r24, in("$25") r25
            }
        }
    };
}

#[naked]
pub unsafe extern "C" fn exception_vec() {
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
