#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]
#![feature(naked_functions)]
#![feature(core_intrinsics)]
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
use psx::hw::cop0::IntSrc;
use psx::hw::{cop0, irq, Register};

fn main() {
    println!("Starting main BIOS function");
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
}
