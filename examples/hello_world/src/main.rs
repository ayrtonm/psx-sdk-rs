#![no_std]
#![no_main]

use psx::println;

#[no_mangle]
fn main() {
    println!("Hello, world!");
    panic!("Ran into some error");
}
