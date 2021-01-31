#![no_std]
#![no_main]

use psx::printf;

#[no_mangle]
fn main() -> ! {
    printf!("Hello, world!\n\0");
    panic!("Ran into some error\0");
    loop {}
}
