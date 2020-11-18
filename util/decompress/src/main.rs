#![feature(min_const_generics)]
#![feature(array_map)]

use std::convert::TryInto;
use std::mem::size_of;

mod codes;
use codes::{Symbol, CODES, SYMBOLS};

fn decompress<const N: usize>() -> [u32; N] {
    // TODO: handle possible misalignment
    //let compressed_exe = include_bytes!("../ferris.tim.zip").chunks(4).map(|x| u32::from_le_bytes(x.try_into().unwrap())).collect::<Vec<u32>>();
    let (a, compressed_exe, b) = unsafe { include_bytes!("../ferris.tim.zip").align_to::<u32>() };
    assert_eq!(a.len(), 0);
    assert_eq!(b.len(), 0);
    let decompressed_len = compressed_exe[0] as usize;
    let mut ret = [0; N];
    // TODO: handle possible misalignment
    let (a, exe, b) = unsafe { ret.align_to_mut::<Symbol>() };
    assert_eq!(a.len(), 0);
    assert_eq!(b.len(), 0);
    let mut possible_code = 0;
    let mut possible_code_len = 0;
    let mut i = 0;
    for &w in &compressed_exe[1..] {
        let mut remaining_bits = 32;
        let mut stream = w as u64 | ((possible_code as u64) << 32);
        while remaining_bits != 0 {
            stream <<= 1;
            remaining_bits -= 1;
            possible_code = (stream >> 32) as u32;
            possible_code_len += 1;
            (&CODES)
                .binary_search(&possible_code)
                .map(|idx| {
                    if i < decompressed_len {
                        ret[i] = SYMBOLS[idx];
                        i += 1;
                        stream &= 0x0000_0000_FFFF_FFFF;
                        possible_code_len = 0;
                    }
                })
                .ok();
        }
        possible_code = (stream >> 32) as u32;
    }
    ret
}

fn main() {
    let ferris = decompress::<32773>()
        .map(|x| x.to_le_bytes())
        .iter()
        .flatten()
        .cloned()
        .collect::<Vec<u8>>();
    std::fs::write("unzipped", ferris).unwrap();
}
