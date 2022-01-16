#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

use core::arch::asm;
use core::mem::forget;
use pretty_hex::{HexConfig, PrettyHex};
use psx::gpu::Vertex;
use psx::sys::gamepad::buttons::*;
use psx::sys::gamepad::{Gamepad, PadType};
use psx::sys::rng::Rng;
use psx::{dprintln, enable_vblank, println, vsync, Font, Framebuffer, KSEG0};

const KB: usize = 1024;
const MB: usize = KB * KB;

#[no_mangle]
fn main() {
    let mut txt = Font::default().text_box(Vertex(0, 0));
    let mut fb = Framebuffer::new(Vertex(0, 0), Vertex(0, 240), Vertex(320, 240)).unwrap();
    let conf = HexConfig {
        title: false,
        ascii: true,
        width: 8,
        group: 4,
        chunk: 2,
    };
    let mut rng = Rng::new(0xdeadbeef);
    enable_vblank();
    let mut buf0 = [0; Gamepad::BUFFER_SIZE];
    let mut buf1 = [0; Gamepad::BUFFER_SIZE];
    let mut pad = Gamepad::new(&mut buf0, &mut buf1).unwrap();
    while pad.info() == PadType::Unknown {}
    let mut start = KSEG0;
    loop {
        let mut start_idx = if start < KSEG0 {
            start = KSEG0;
            0
        } else {
            start - KSEG0
        };
        let mut end = start_idx + 232;
        if end >= 2 * MB {
            end = 2 * MB;
        }
        // UNSAFETY: dprintln! creates a `&mut txt` while we have a shared reference to all of main
        // RAM (which includes the memory used by `txt`) so this is undefined behavior. The
        // demo generally produces the expected result, but there is no expectation that it will
        // always work and this program should not be relied on for correctness.
        let all_memory = unsafe { core::slice::from_raw_parts(KSEG0 as *const u8, 2 * MB) };
        // Even if we copied the viewport memory using pointers, undefined behavior (for some range
        // of addresses) is unavoidable so let's take the easy way out.
        let viewport = &all_memory[start_idx..end];
        dprintln!(txt, "Addr: {:p}", viewport.as_ptr());
        dprintln!(txt, "{:?}", viewport.hex_conf(conf));
        // A best-effort workaround for limiting the overlap between the shared reference to main
        // RAM and other mutable references. This does not cancel out the fact that the whole
        // program exhibits undefined behavior.
        forget(all_memory);

        let buttons = pad.poll();
        if buttons.pressed(UP) {
            start -= 8;
        } else if buttons.pressed(DOWN) {
            start += 8;
        } else if buttons.pressed(LEFT) {
            start -= 232;
        } else if buttons.pressed(RIGHT) {
            start += 232;
        } else if buttons.pressed(CROSS) {
            extern "C" {
                static STACK_INIT: u8;
            }
            unsafe {
                start = (&STACK_INIT as *const u8 as usize) - 232;
            }
            println!("Jumping to the bottom of the stack");
        } else if buttons.pressed(TRIANGLE) {
            let sp: usize;
            unsafe {
                asm!("move {}, $29", out(reg) sp);
            }
            start = sp;
            println!("Jumping to the top of the stack");
        } else if buttons.pressed(SQUARE) {
            extern "C" {
                static __data_start: u8;
            }
            unsafe {
                start = &__data_start as *const u8 as usize;
            }
            println!("Jumping to the top of .data");
        } else if buttons.pressed(CIRCLE) {
            extern "C" {
                static __bss_start: u8;
            }
            unsafe {
                start = &__bss_start as *const u8 as usize;
            }
            println!("Jumping to the top of .bss");
        } else if buttons.pressed(START) {
            extern "C" {
                static __text_start: u8;
            }
            unsafe {
                start = &__text_start as *const u8 as usize;
            }
            println!("Jumping to the start of .text");
        } else if buttons.pressed(SELECT) {
            extern "C" {
                static __text_end: u8;
            }
            unsafe {
                start = &__text_end as *const u8 as usize - 232;
            }
            println!("Jumping to the end of .text");
        }
        txt.reset();
        vsync();
        fb.swap(None);
    }
}
