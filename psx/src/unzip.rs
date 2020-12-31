use core::hint::unreachable_unchecked;

use crate::workarounds::{get_unchecked, get_unchecked_mut, get_unchecked_slice,
                         get_unchecked_slice_from};

// TODO: why is unchecked ok here?
/// Decompresses a zipped file. Not intended to be called directly. Use
/// [`crate::unzip!`] instead.
pub const unsafe fn unzip<const M: usize, const N: usize>(zip: [u32; M]) -> [u32; N] {
    // Copied from standard library, but modified to disable panic checks
    const unsafe fn binary_search(slice: &[u32], x: u32) -> Option<usize> {
        let mut size = slice.len();
        if size == 0 {
            unreachable_unchecked();
        };
        let mut base = 0;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            let cmp = *get_unchecked(slice, mid) > x;
            base = if cmp { base } else { mid };
            size -= half;
        }
        if *get_unchecked(slice, base) == x {
            Some(base)
        } else {
            None
        }
    }
    // The zip file starts with 8 bits containing the number of symbols to unzip (as
    // a u32) followed by the number of entries in the dictionary (as a u32).
    let num_symbols = *get_unchecked(&zip, 0) as usize;
    let num_entries = *get_unchecked(&zip, 1) as usize;

    // Then come the dictionary codes
    let code_start = 2;
    let code_end = code_start + num_entries;
    let codes = get_unchecked_slice(&zip, code_start..code_end);

    // The come the corresponding dictionary symbols
    let sym_start = code_end;
    let sym_end = sym_start + num_entries;
    let symbols = get_unchecked_slice(&zip, sym_start..sym_end);

    // Finally we reach the compressed data
    let file_start = sym_end;
    let mut ret = [0; N];
    let mut possible_code = 0;
    let mut possible_code_len = 0;
    let mut ret_idx = 0;
    let mut i = 0;
    let remaining_data = get_unchecked_slice_from(&zip, file_start..);
    while i < remaining_data.len() {
        let word = remaining_data[i];
        i += 1;
        let mut remaining_bits = 32;
        let mut stream = word as u64 | ((possible_code as u64) << 32);
        while remaining_bits != 0 {
            stream <<= 1;
            remaining_bits -= 1;
            possible_code_len += 1;
            possible_code = (stream >> 32) as u32;
            // TODO: Replace with const Option::map
            match binary_search(codes, possible_code | (1 << possible_code_len)) {
                Some(idx) => {
                    if ret_idx < num_symbols {
                        *get_unchecked_mut(&mut ret, ret_idx) = *get_unchecked(symbols, idx);
                        ret_idx += 1;
                        stream &= 0x0000_0000_FFFF_FFFF;
                        possible_code_len = 0;
                    }
                },
                None => (),
            };
        }
        possible_code = (stream >> 32) as u32;
    }
    ret
}
