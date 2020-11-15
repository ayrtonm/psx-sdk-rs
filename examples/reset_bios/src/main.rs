#![no_std]
#![no_main]
#![feature(asm)]

libpsx::exe!();

fn main(mut _ctxt: Ctxt) {
    unsafe {
        asm!("lui $3, 0xbfc0
              jr $3");
    }
}
