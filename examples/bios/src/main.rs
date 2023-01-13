#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]
#![feature(naked_functions)]
#![feature(core_intrinsics)]
#![feature(const_mut_refs)]
#![feature(const_ptr_as_ref)]
#![feature(const_option)]
#![feature(inline_const)]

mod allocator;
mod boot;
mod exceptions;
mod gamepad;
mod global;
mod handlers;
mod misc;
mod rand;
mod stdout;
mod thread;

use crate::misc::{get_system_date, get_system_version};
use crate::thread::Thread;
use psx::hw::cop0::IntSrc;
use psx::hw::{cop0, irq, Register};

fn main() {
    // This main loop doesn't do anything useful yet, it's only used to test
    // functionality that would be exposed to executables if the BIOS could load
    // them
    println!("Starting main BIOS loop");
    let version_str = get_system_version();
    let bios_date = get_system_date();
    println!("{:?}", version_str);
    println!("{:x?}", bios_date);

    cop0::Status::new()
        .enable_interrupts()
        .unmask_interrupt(IntSrc::Hardware)
        .use_boot_vectors(false)
        .store();
    irq::Status::skip_load().ack_all().store();
    irq::Mask::new().enable_all().store();

    gamepad::init();

    let mut t = Thread::create(task).unwrap();
    t.resume();
    let mut x = 0;
    loop {
        x += 1;
        if x > 2000 {
            t.unpark();
        }
        println!("hello from main thread");
    }
}

extern "C" fn task() {
    let mut x = 0;
    loop {
        x += 1;
        if x > 1000 {
            thread::park();
        }
        println!("hello from task");
    }
}
