#![no_std]
#![no_main]
#![feature(asm, naked_functions)]

use core::mem::transmute;
use core::ptr::write_volatile;

use psx::cop0;
use psx::framebuffer::UnsafeFramebuffer;
use psx::interrupt;
use psx::interrupt::IRQ;
use psx::mmio::MMIO;
use psx::printer::UnsafePrinter;

static mut NUM: usize = 0;
const TASKS: [fn(); 2] = [do_this, do_that];

#[no_mangle]
fn main(mut mmio: MMIO) -> ! {
    install_vector(scheduler);
    mmio.irq_mask
        .get_mut()
        .disable_all()
        .enable(IRQ::Vblank)
        .set();
    cop0::Status::read().set(cop0::Status::IM_HW).write();
    cop0::Status::read().enable_interrupts().write();
    loop {}
}

fn do_this() {
    let mut f = UnsafeFramebuffer::default();
    let mut p = UnsafePrinter::<1024>::default();
    p.load_font();
    p.print(b"doing this", []);
    f.swap();
    psx::delay(2000000);
}

fn do_that() {
    let mut f = UnsafeFramebuffer::default();
    let mut p = UnsafePrinter::<1024>::default();
    p.load_font();
    p.print(b"doing that", []);
    f.swap();
    psx::delay(2000000);
}

fn idle() {
    let mut f = UnsafeFramebuffer::default();
    let mut p = UnsafePrinter::<1024>::default();
    p.load_font();
    loop {
        p.print(b"doing nothing\n", []);
        f.swap();
        psx::delay(2000000);
    }
}

#[naked]
fn scheduler(mut mmio: MMIO) {
    interrupt::free(|| unsafe {
        if NUM < TASKS.len() {
            NUM += 1;
            TASKS[NUM - 1]();
        } else {
            idle();
        }
    });
    cop0::Status::read().set(cop0::Status::IM_HW).write();
    loop {}
}

fn install_vector<T>(f: fn(T)) {
    unsafe {
        let handler_addr = transmute::<_, u32>(f as fn(_));
        let j = (2 << 26) | ((handler_addr & 0x03FF_FFFF) >> 2);
        write_volatile(0x8000_0080 as *mut u32, j);
        write_volatile(0x8000_0084 as *mut u32, 0);
    }
}
