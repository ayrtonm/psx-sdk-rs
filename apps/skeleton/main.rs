#![feature(no_std,core)]
#![no_std]

use core::intrinsics::{volatile_store, volatile_load};
use core::fmt::Write;
use psx::uart::Uart;

#[macro_use]
extern crate core;
extern crate psx;

#[no_mangle]
pub fn main() {
    //let mut uart = Uart::new();

    // unsafe { bios_print_devices() };
    // print_devices();
    // unsafe { bios_putchar(b'$') };
    // putchar(b'$');
    // unsafe { bios_print_devices() };

    // putchar(b'A');
    // printf(b"test %d\n\0" as *const u8, 42);
    
    // unsafe { bios_puts(b"test %d\n\0" as *const u8); }

    // putchar(b'A');
    // putchar(b'B');
    // putchar(b'C');

    // printf(b"Foo %d\n\0" as *const u8, 128);

    //let _ = writeln!(uart, "Hello world from rust!");

    //for &b in b"\n\nHello world from rust!\n\n" {
    Uart::putchar(b'$');
    Uart::putchar(b'$');
    //Uart::putchar(b'$');
        //putchar(b);
    //}
    //

    //uart.putchar(c);

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

    //loop {}
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
    let cmd_reg = 0x1f801814u32 as *mut u32;

    unsafe {
        let v = volatile_load(cmd_reg);

        volatile_store(cmd_reg, (v != cmd_reg as u32) as u32);
    }
}

fn delay(n: u32) {
    for _ in 0..n {
        unsafe {
            volatile_load(0 as *mut u32);
        }
    }
}

fn print_devices() {
    unsafe {
        bios_print_devices();
    }
}

fn putchar(c: u8) {
    unsafe {
        bios_putchar(c);
    }
}

fn printf(c: *const u8, v: u32) {
    unsafe { bios_printf(c, v) };
}

extern {
    fn bios_putchar(b: u8) -> u32;
    fn bios_puts(s: *const u8) -> u32;
    fn bios_toupper(b: u8) -> u8;
    fn bios_print_devices();
    fn bios_printf(s: *const u8, v: u32);
}
