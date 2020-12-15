#![no_std]
#![no_main]

use core::any::Any;
use core::mem::{size_of_val, transmute};

use psx::mmio::{int, MMIO};

use psx::framebuffer::UnsafeFramebuffer;
use psx::printer::UnsafePrinter;

use psx::gpu::{Color, Vertex};
use psx::interrupt::IRQ;
use psx::{cop0, interrupt};

#[no_mangle]
fn main(mut mmio: MMIO) {
    let mut p = UnsafePrinter::<1024>::default();
    let mut f = UnsafeFramebuffer::default();
    p.load_font();
    run_tests(&mut mmio);
    unsafe {
        let exception_addr: u32 = transmute(exception as fn());
        let j = (1 << 31) | ((exception_addr & 0x0FFF_FFFF) / 4);
        p.print(b"entry point {}\n{}\n", [j, exception_addr]);
    }
    p.print(b"All tests passed", []);
    f.swap();
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

fn exception() {
    let mut p = UnsafePrinter::<1024>::default();
    let mut f = UnsafeFramebuffer::default();
    p.load_font();
    p.print(b"\nhit an exception", []);
    f.swap();
    loop {}
}

fn test_exception(mmio: &mut MMIO) {
    unsafe {
        let exception_addr = transmute::<_, u32>(exception as fn());
        let j = (2 << 26) | ((exception_addr & 0x03FF_FFFF) >> 2);
        core::ptr::write_volatile(0x8000_0080 as *mut u32, j);
        // Don't forget to fill the jump delay slot
        core::ptr::write_volatile(0x8000_0084 as *mut u32, 0);
        let mut stat = cop0::Status::read();
        stat.remove(cop0::Status::BEV);
        stat.write();
        mmio.int_mask.disable_all();
        interrupt::enable();
        interrupt::free(|| {
            mmio.int_mask.enable(IRQ::Vblank);
            mmio.int_stat.ack(IRQ::Vblank);
        });
        //exception();
        loop {}
    }
}
