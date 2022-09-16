#![no_std]
#![no_main]

use core::mem::size_of;
use psx::constants::*;
use psx::sys::fs::{File, CDROM};
use psx::sys::kernel::{psx_do_execute, psx_flush_cache};
use psx::{dprintln, file_size, Framebuffer};

#[no_mangle]
fn main() {
    let mut fb = Framebuffer::default();
    let mut txt = fb.load_default_font().new_text_box((0, 8), (320, 240));
    loop {
        dprintln!(txt, "Running the ferris demo...");
        fb.swap();
        delay(5000000);

        // Open the executable file on the CD
        let file = File::<CDROM>::open("cdrom:\\PROG2.EXE").expect("Could not find PROG2.EXE");

        // Get the executable size in bytes at compile-time
        const EXE_SIZE: usize =
            file_size!("../../ferris/target/mipsel-sony-psx/release/ferris.exe");

        // Add the executable load offset to the end of the BIOS region (i.e. MAIN_RAM +
        // BIOS_LEN). The load offset refers to start of the executable without the
        // header so we have to subtract 2KB to get the actual load address.
        let load_addr = 524288 + MAIN_RAM as usize + BIOS_LEN - 2048;

        // Create a mutable reference to the memory where the executable will be loaded
        // SAFETY: No references to this memory overlap the lifetime of this slice in
        // this executable.
        let exe = unsafe {
            core::slice::from_raw_parts_mut(load_addr as *mut u32, EXE_SIZE / size_of::<u32>())
        };

        // Read the CD file into the memory it will run from
        file.read(exe).expect("Could not read PROG2.EXE");

        // SAFETY: flush_cache has no safety requirements. psx_do_execute was given a
        // pointer to the header of a valid executable.
        unsafe {
            psx_flush_cache();
            let init_pc_offset = 4;
            psx_do_execute(&mut exe[init_pc_offset] as *mut u32 as *mut u8, 0, 0);
        }

        // Clear whatever the demo had on the screen
        fb.swap();
        // The ferris demo doesn't clobber the VRAM containing our font
        dprintln!(txt, "Returned from the ferris demo");
        fb.swap();
        delay(5000000);
    }
}

fn delay(n: usize) {
    for _ in 0..n {
        unsafe {
            core::ptr::read_volatile(0 as *const u32);
        }
    }
}
