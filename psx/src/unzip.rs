//! Unzipping routine for data compressed using Huffman coding
use crate::std::{binary_search, slice, slice_from};

pub unsafe fn unzip<const IN: usize, const OUT: usize>(zip: [u32; IN]) -> [u32; OUT] {
    internal_unzip(zip)
}

const fn internal_unzip<const IN: usize, const OUT: usize>(zip: [u32; IN]) -> [u32; OUT] {
    let num_symbols = zip[0] as usize;
    let num_entries = zip[1] as usize;

    let code_start = 2;
    let code_end = code_start + num_entries;
    let codes = unsafe { slice(&zip, code_start..code_end) };

    let sym_start = code_end;
    let sym_end = sym_start + num_entries;
    let symbols = unsafe { slice(&zip, sym_start..sym_end) };

    let file_start = sym_end;
    let mut ret = [0; OUT];
    let mut possible_code = 0;
    let mut possible_code_len = 0;
    let mut ret_idx = 0;
    let remaining_data = unsafe { slice_from(&zip, file_start..) };
    const_iter! {
        word in remaining_data => {
        let mut remaining_bits = 32;
        let mut stream = word as u64 | ((possible_code as u64) << 32);
        while remaining_bits != 0 {
            stream <<= 1;
            remaining_bits -= 1;
            possible_code_len += 1;
            possible_code = (stream >> 32) as u32;
            if ret_idx < num_symbols {
                let value = possible_code | (1 << possible_code_len);
                match binary_search(&codes, value) {
                    Some(idx) => {
                        ret[ret_idx] = symbols[idx];
                        ret_idx += 1;
                        stream &= 0x0000_0000_FFFF_FFFF;
                        possible_code_len = 0;
                    },
                    None => (),
                }
            }
        }
        possible_code = (stream >> 32) as u32;
    }};
    ret
}

#[cfg(test)]
mod tests {
    use super::internal_unzip;

    #[allow(dead_code)]
    const FONT_TEST: () = {
        const N: usize = unzipped_size!("../font.tim.zip");
        let unzipped = internal_unzip(include_u32!("../font.tim.zip")) as [u32; N];
        let original: [u32; N] = include_u32!("../font.tim");
        assert!(slice_cmp!(original, unzipped));
    };
}
