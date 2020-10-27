#![feature(core_intrinsics)]
#![no_std]
#![no_main]

#[allow(unused_imports)]
#[macro_use]
extern crate core;

use core::intrinsics::volatile_load;

#[no_mangle]
pub fn main() {
    let max_alpha = 1.0;
    let min_alpha = 0.0;
    let mut delta = 1.0 / 255.0;
    let mut alpha = min_alpha;
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
    // Clear command FIFO
    libpsx::bios::gpu_gp1_command_word(0x01000000);
    // Top left at 0,0
    libpsx::bios::gpu_command_word(0xe3000000);
    // Bottom right: 256x256
    libpsx::bios::gpu_command_word(0xe4080100);
    // Offset at 0,0
    libpsx::bios::gpu_command_word(0xe5000000);
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
    libpsx::bios::gpu_command_word_params(&quad[0], 8);
}

fn blink() {
    delay(50000);
}

fn delay(n: u32) {
    for _ in 0..n {
        unsafe {
            volatile_load(0 as *mut u32);
        }
    }
}
