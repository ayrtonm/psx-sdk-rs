#![no_std]
#![no_main]
#![feature(core_intrinsics, asm)]

libpsx::exe!();

#[link_section = ".comment"]
fn main(mut _ctxt: Ctxt) {
    let bytes = include_bytes!("../../rotating_square/rotating_square.psexe");
    fn read_word(ar: &[u8]) -> u32 {
        ar[0] as u32 | (ar[1] as u32) << 8 | (ar[2] as u32) << 16 | (ar[3] as u32) << 24
    }
    let dest = read_word(&bytes[0x18..]);
    //size in 32-bit words
    let size = read_word(&bytes[0x1c..]) >> 2;
    for n in 0..size {
        unsafe {
            let addr = (dest + (n << 2)) as *mut u32;
            let idx = 0x800 + ((n as usize) << 2);
            let val = read_word(&bytes[idx..]);
            core::intrinsics::volatile_store(addr, val);
        }
    }
    unsafe {
        asm!("andi $28, 0
              lui $29, 0x801f
              ori $29, 0xfff0
              lui $30, 0x801f
              ori $30, 0xfff0
              lui $3, 0x8001
              jr $3");
    }
}
