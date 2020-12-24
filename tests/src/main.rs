#![no_std]
#![no_main]

use psx::compatibility::*;

#[no_mangle]
fn main() {
    reset_graph();
}
