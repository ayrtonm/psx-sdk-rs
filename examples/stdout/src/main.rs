#![no_std]
#![no_main]

use psx::sys::kernel;
use psx::sys::rng::Rng;
use psx::{printf, print, println};

// Printing to stdout was tested with SCPH7001 on mednafen with `psx.dbg_level 2` in mednafen.cfg
// Duckstation is also known to work with this BIOS.
// Other BIOS/emulator combinations may or may not work
#[no_mangle]
fn main() {
    // Seed the BIOS RNG to print some random numbers
    let rng = Rng::new(0xdeadbeef);

    // The raw BIOS function can be used to print to stdout
    // Format strings can be variables and they must be a pointer (i.e. *const i8).
    let fmt_str = "Hello, %s! 0x%x\n\0".as_ptr() as *const i8;
    // Args formatted with `%s` must also be pointers even though it can't be enforced by the type-system.
    let str_arg = "world\0".as_ptr();
    let rand_num: u32 = rng.rand();
    // SAFETY: Format strings and arguments formatted with '%s' are explicitly null-terminated
    unsafe {
        kernel::printf(fmt_str, str_arg, rand_num);
    }

    // The printf! macro adds null-terminators where necessary to remove the need for unsafe. This
    // will only call the BIOS once, but may do some copying if the format string and `%s` args
    // are not explicitly null-terminated.
    let printf_msg = "%s null-terminates its args if necessary 0x%x\n";
    let macro_name = b"`printf!`";
    printf!(printf_msg, macro_name, rng.rand::<u32>());

    // The print! and println! macros work just like the std versions, but may call the BIOS
    // multiple times.
    let rust_macro_names = "print! and println!";
    print!("Rust-style formatting is also available with {}\n", rust_macro_names);
    println!("These macros require a literal fmt string though {:#x?}", rng.rand::<u32>());
    loop {}
}
