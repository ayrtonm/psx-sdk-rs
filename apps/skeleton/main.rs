#![feature(no_std,core)]
#![no_std]

use core::intrinsics::{volatile_store, volatile_load};
use psx::uart::Uart;

#[macro_use]
extern crate core;
extern crate psx;

#[no_mangle]
pub fn main() {
    let uart = Uart::new();

    for &b in b"\n\nHello world from rust!\n\n" {
        uart.putc(b);
    }

    // Clear command FIFO
    gp1_command(0x01000000);

    // Top left at 0,0
    gp0_command(0xe3000000);
    // Bottom right: 256x256
    gp0_command(0xe4040100);
    // Offset at 0,0
    gp0_command(0xe5000000);

    // Shaded quad
    gp0_command(0x38000000);
    gp0_command(0x00000000);
    gp0_command(0x00ff0000);
    gp0_command(0x00000100);
    gp0_command(0x0000ff00);
    gp0_command(0x01000000);
    gp0_command(0x000000ff);
    gp0_command(0x01000100);

    //delay(500000);
}

/// Send command on GPU port 0
fn gp0_command(cmd: u32) {
    let cmd_reg = 0x1f801810u32 as *mut u32;

    // Hack to avoid overflowing the command FIFO, I should check the
    // ready status flag.
    delay(100);

    unsafe {
        volatile_store(cmd_reg, cmd);
    }
}

/// Send command on GPU port 1
fn gp1_command(cmd: u32) {
    let reg = 0x1f801814u32 as *mut u32;

    unsafe {
        volatile_store(reg, cmd);
    }
}

fn delay(n: u32) {
    for _ in 0..n {
        unsafe {
            volatile_load(0 as *mut u32);
        }
    }
}
