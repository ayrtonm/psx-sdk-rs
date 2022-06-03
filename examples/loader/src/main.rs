#![no_std]
#![no_main]

use psx::sys::kernel::{do_execute, flush_cache};
use psx::{dprintln, Framebuffer};

#[no_mangle]
fn main() {
    loop {
        let mut fb = Framebuffer::default();
        let mut txt = fb.load_default_font().new_text_box((0, 8), (320, 240));
        dprintln!(txt, "Running the ferris demo...");
        fb.swap();
        delay(5000000);

        let exe = include_bytes!("../../ferris/target/mipsel-sony-psx/release/ferris.exe");
        let load_addr_offset = 6 * 4;
        let exe_size_offset = 7 * 4;
        let header_size = 0x800;
        // This field is the load address of the executable without the header. The
        // complete exe (including the header) is loaded 2KB before the load address.
        let load_addr = usize::from_le_bytes(
            exe[load_addr_offset..load_addr_offset + 4]
                .try_into()
                .unwrap(),
        ) - header_size;
        // This field is the size of the executable without the header. The complete exe
        // includes a 2KB header.
        let exe_size = usize::from_le_bytes(
            exe[exe_size_offset..exe_size_offset + 4]
                .try_into()
                .unwrap(),
        ) + header_size;
        // SAFETY: No references to this memory overlap the lifetime of this slice in
        // this executable.
        let loaded_exe = unsafe { core::slice::from_raw_parts_mut(load_addr as *mut u8, exe_size) };
        // Copy the executable from .data to the address it will be executed from.
        // Normally the executable would be loaded directly from disk to its load
        // address, but that's currently out of the scope of this demo.
        loaded_exe.copy_from_slice(exe);

        // SAFETY: flush_cache has no safety requirements. do_execute was given a
        // pointer to the header of a valid executable.
        unsafe {
            flush_cache();
            let init_pc_offset = 4 * 4;
            do_execute(&mut loaded_exe[init_pc_offset], 0, 0);
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
