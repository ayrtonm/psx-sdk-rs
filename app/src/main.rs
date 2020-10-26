#![feature(start,core_intrinsics)]
#![no_std]
#![no_main]

use core::intrinsics::{volatile_store, volatile_load};
use core::fmt::Write;

#[macro_use]
extern crate core;
extern crate psx;

#[no_mangle]
pub fn main() {
    let max_alpha = 1.0;
    let min_alpha = 0.0;
    let mut delta = 1.0 / 255.0;
    let mut alpha = min_alpha;
    let mut x: u8 = 0;
    psx::memset(&mut x as *mut u8, 0, 0);
    loop {
        draw(alpha);
        alpha += delta;
        if alpha > max_alpha || alpha < min_alpha {
            delta *= -1.0;
        };
        blink();
    }
}

fn draw(alpha: f32) {
    unsafe {
        // Clear command FIFO
        bios_gpu_gp1_command_word(0x01000000);
        // Top left at 0,0
        bios_gpu_command_word(0xe3000000);
        // Bottom right: 256x256
        bios_gpu_command_word(0xe4080100);
        // Offset at 0,0
        bios_gpu_command_word(0xe5000000);
        // Shaded quad
        let alpha = (255.0* alpha / 1.0) as u32;
        let cmd = 0x38 << 24;
        let top_left = 0x00000000;
        let top_right = 0x00000100;
        let bottom_left = 0x01000000;
        let bottom_right = 0x01000100;
        let black = 0x00_000000;
        let blue = alpha << 16;
        let green = alpha << 8;
        let red = alpha;
        let quad = [cmd | blue, top_left,
                    green, top_right,
                    red, bottom_left,
                    black, bottom_right,
        ];
        bios_gpu_command_word_and_params(&quad[0], 8);
        load_delay_test();
    }
}

fn blink() {
    delay(20000);
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

    delay(100);
    unsafe {
        //let v = volatile_load(cmd_reg);

        //volatile_store(cmd_reg, (v != cmd_reg as u32) as u32);
        volatile_store(cmd_reg, cmd);
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
    fn bios_gpu_get_status() -> u32;
    fn bios_gpu_gp1_command_word(cmd: u32);
    fn bios_gpu_command_word(cmd: u32);
    fn bios_gpu_command_word_and_params(src: *const u32, num: u32);
    fn load_delay_test();
}
