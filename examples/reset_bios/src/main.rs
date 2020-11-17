#![no_std]
#![no_main]
#![feature(asm)]

psx::exe!();

fn main(mut _io: IO) {
    unsafe {
        asm!(
            "lui $3, 0xbfc0
              jr $3"
        );
    }
}
