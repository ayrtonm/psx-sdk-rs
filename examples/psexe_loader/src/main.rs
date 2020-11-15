#![no_std]
#![no_main]
#![feature(core_intrinsics, asm)]

mod huffman_code;
use huffman_code::{CODES, SYMBOLS};

libpsx::exe!();

#[link_section = ".exe_loader"]
fn main(mut _ctxt: Ctxt) {
    //let bytes = include_bytes!("../rotating_square.psexe");
    let bytes = &load_game();
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

#[link_section = ".exe_loader"]
fn load_game() -> [u8; 14336] {
    let compressed_exe = include_bytes!("../rotating_square.psexe.hzip");
    let mut exe = [0; 14336];
    let mut i = 0;
    let mut possible_code_len = 0;
    let mut possible_code;
    for w in compressed_exe.chunks(2) {
        let mut stream = (w[0] as u32) | (w[1] as u32) << 8;
        while stream != 0 {
            stream <<= 1;
            possible_code_len += 1;
            possible_code = (stream >> 16) as u16 | (1 << possible_code_len);
            CODES.iter().position(|&code| code == possible_code).map(|idx| {
                let symbol = SYMBOLS[idx];
                exe[i] = symbol;
                i += 1;
                possible_code_len = 0;
                stream &= 0x0000_ffff;
            });
        }
    }
    exe
}
