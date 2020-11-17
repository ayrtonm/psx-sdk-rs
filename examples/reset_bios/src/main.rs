#![no_std]
#![no_main]
#![feature(asm)]

psx::exe!();

fn main(mut _ctxt: Ctxt) {
    unsafe {
        asm!(
            "lui $3, 0xbfc0
              jr $3"
        );
    }
}
