mod huffman_code;
use crate::huffman_code::{CODES, SYMBOLS};

fn main() {
    let compressed_exe = include_bytes!("../../rotating_square.psexe.hzip");
    let mut exe = Vec::new();
    let mut possible_code_len = 0;
    let mut possible_code;
    for w in compressed_exe.chunks(2) {
        let mut remaining_bits = 16;
        let mut stream = (w[0] as u32) | (w[1] as u32) << 8;
        while remaining_bits != 0 {
            stream <<= 1;
            remaining_bits -= 1;
            possible_code_len += 1;
            possible_code = (stream >> 16) as u16 | (1 << possible_code_len);
            CODES.iter().position(|&code| code == possible_code).map(|idx| {
                let symbol = SYMBOLS[idx];
                exe.push(symbol);
                possible_code_len = 0;
                stream &= 0x0000_ffff;
            });
        }
    }
    std::fs::write("rotating_square.psexe.hunzip", exe).unwrap();
}
