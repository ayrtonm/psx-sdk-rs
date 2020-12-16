#![no_std]
#![no_main]
#![feature(asm, naked_functions)]

use core::any::Any;
use core::mem::{size_of_val, transmute};

use psx::mmio::{int, MMIO};

use psx::framebuffer::{Framebuffer, UnsafeFramebuffer};
use psx::printer::UnsafePrinter;

use psx::gpu::{Color, Vertex};
use psx::interrupt::IRQ;
use psx::{cop0, interrupt};

#[no_mangle]
fn main(mut mmio: MMIO) {
    run_tests(&mut mmio);
    tests_passed();
}

fn tests_passed() {
    let mut p = UnsafePrinter::<1024>::default();
    let mut f = UnsafeFramebuffer::default();
    p.load_font();
    p.print(b"All tests passed", []);
    f.swap();
    loop {}
}

fn run_tests(mmio: &mut MMIO) {
    check_sizes(mmio);
    test_int_mask(&mut mmio.int_mask);
    test_exception(mmio);
}

fn check_sizes(mmio: &mut MMIO) {
    let value_sizes = [
        (mmio as &dyn Any, 0),
        (&Vertex::from(0) as &dyn Any, 4),
        (&Color::BLUE as &dyn Any, 3),
    ];
    for (val, size) in &value_sizes {
        assert!(size_of_val(*val) == *size);
    }
}

fn test_int_mask(int_mask: &mut int::Mask) {
    int_mask.disable_all();
    int_mask.enable(IRQ::Vblank);
    let mut enabled = int_mask.enabled();
    assert!(enabled.next() == Some(IRQ::Vblank));
    assert!(enabled.next().is_none());
}

// Required to return from the exception
#[naked]
fn exception(mut mmio: MMIO) {
    let gp0 = &mut mmio.gp0;
    let gp1 = &mut mmio.gp1;
    let mut p = UnsafePrinter::<1024>::default();
    let mut f = Framebuffer::new(0, (0, 240), (320, 240), gp0, gp1);
    p.load_font();
    unsafe {
        p.print(
            b"hit an exception\n\
                  EPC (cop0r14) contains {}\n\
                  Entry point {}\n\
                  test_exception {}\n\
                  this fn {}\n\
                  end fn {}",
            [
                cop0::EPC::read(),
                transmute(main as fn(_)),
                transmute(test_exception as fn(_)),
                transmute(exception as fn(_)),
                transmute(tests_passed as fn()),
            ],
        );
    }
    f.swap(gp0, gp1);
    interrupt::disable();
    let mut stat = cop0::Status::read();
    stat.remove(cop0::Status::IM);
    stat.write();
    unsafe {
        asm!("j $2
              nop", in("$2") cop0::EPC::read());
    }
}

fn test_exception(mmio: &mut MMIO) {
    unsafe {
        let exception_addr = transmute::<_, u32>(exception as fn(_));
        let j = (3 << 26) | ((exception_addr & 0x03FF_FFFF) >> 2);
        core::ptr::write_volatile(0x8000_0080 as *mut u32, j);
        // Don't forget to fill the jump delay slot
        core::ptr::write_volatile(0x8000_0084 as *mut u32, 0);
        let mut stat = cop0::Status::read();
        stat.remove(cop0::Status::BEV);
        stat.insert(cop0::Status::IM);
        stat.write();
        interrupt::enable();
        // This function should return after the rfe so end() shouldn't be
        // necessary, but it seems that the interrupt handler is messing with
        // $ra or my stack frames
        //end();
    }
}
