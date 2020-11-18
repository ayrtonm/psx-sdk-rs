mod huffman_code;
use crate::huffman_code::{CODES, SYMBOLS};

fn main() {
    let compressed_exe = include_bytes!("../../ferris.tim.hzip");
    let mut exe = Vec::new();
    let mut possible_code_len = 0;
    let mut possible_code = 0;
    for w in compressed_exe.chunks(4) {
        let mut remaining_bits = 32;
        let mut stream = (w[0] as u64) | (w[1] as u64) << 8 | (w[2] as u64) << 16 | (w[3] as u64) << 24 | ((possible_code as u64) << 32);
        while remaining_bits != 0 {
            stream <<= 1;
            remaining_bits -= 1;
            possible_code_len += 1;
            possible_code = (stream >> 32) as u32;
            CODES.iter().position(|&code| code == possible_code).map(|idx| {
                let symbol = SYMBOLS[idx];
                exe.push(symbol);
                println!("found symbol {:#x} with code {:#04x}", symbol, possible_code);
                possible_code_len = 0;
                stream &= 0x0000_0000_FFFF_FFFF;
            });
        }
        possible_code = (stream >> 32) as u32;
        println!("resetting with {:#018b} {}", possible_code, possible_code_len);
    }
    std::fs::write("ferris.tim.hunzip", exe).unwrap();
}
