#![no_std]
#![no_main]

use core::slice;
use pretty_hex::{HexConfig, PrettyHex};
use psx::println;
use psx::sys::gamepad::{Buffer, Button, GamePad};

#[no_mangle]
fn main() {
    const MAX_LEN: usize = 0x350;
    let ram_start = psx::KSEG0 as *const u8;
    let text_start = {
        extern "C" {
            static __text_start: u8;
        }
        unsafe { &__text_start as *const u8 }
    };
    let heap_start = {
        extern "C" {
            static __heap_start: u8;
        }
        unsafe { &__heap_start as *const u8 }
    };
    let stack_init = {
        extern "C" {
            static STACK_INIT: u8;
        }
        unsafe { &STACK_INIT as *const u8 }
    };

    let mut len: usize = 0x100;
    let mut start = text_start;

    let mut buf0 = Buffer::new();
    let mut buf1 = Buffer::new();
    let pad = GamePad::new(&mut buf0, &mut buf1);

    let mut conf = HexConfig {
        title: false,
        ascii: true,
        width: 16,
        group: 8,
        chunk: 1,
    };

    let print_memory = |start, len| {
        let memory = unsafe { slice::from_raw_parts(start, len) };
        println!(
            "Address: {:p}, length: {} ({1:x}) bytes",
            memory.as_ptr(),
            memory.len()
        );
        println!("{:?}", memory.hex_conf(conf));
    };

    print_memory(start, len);

    loop {
        let old_start = start;
        let old_len = len;

        // Check if we need to update start address
        if pad.pressed(Button::Up) {
            if start != ram_start {
                start = unsafe { start.sub(len / 2) };
            }
        } else if pad.pressed(Button::Down) {
            if start != stack_init {
                start = unsafe { start.add(len / 2) };
            }
        } else if pad.pressed(Button::Cross) {
            start = stack_init;
        } else if pad.pressed(Button::Triangle) {
            start = ram_start;
        } else if pad.pressed(Button::Square) {
            start = text_start;
        } else if pad.pressed(Button::Circle) {
            start = heap_start;
        }

        // Check if we need to update length
        if pad.pressed(Button::Left) {
            if len != 0 {
                len -= 0x10;
            }
        } else if pad.pressed(Button::Right) {
            if len != MAX_LEN {
                len += 0x10;
            }
        }

        // Print to stdout only if slice changed
        if old_start != start || old_len != len {
            print_memory(start, len);
        }
    }
}
