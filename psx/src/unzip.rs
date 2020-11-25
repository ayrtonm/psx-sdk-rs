pub fn decompress<const M: usize, const N: usize>(zip: [u32; M]) -> [u32; N] {
    // The zip file contains an 8-bit with the number of symbols to unzip (as a u32)
    // followed by the number of entries in the dictionary (as a u32).
    let num_symbols = zip[0] as usize;
    let num_entries = zip[1] as usize;

    // Then come the dictionary codes
    let code_start = 2;
    let code_end = code_start + num_entries;
    let codes = &zip[code_start..code_end];

    // The come the corresponding dictionary symbols
    let sym_start = code_end;
    let sym_end = sym_start + num_entries;
    let symbols = &zip[sym_start..sym_end];

    // Finally we reach the compressed data
    let file_start = sym_end;
    let mut ret = [0; N];
    let mut possible_code = 0;
    let mut possible_code_len = 0;
    let mut ret_idx = 0;
    for &word in &zip[file_start..] {
        let mut remaining_bits = 32;
        let mut stream = word as u64 | ((possible_code as u64) << 32);
        while remaining_bits != 0 {
            stream <<= 1;
            remaining_bits -= 1;
            possible_code_len += 1;
            possible_code = (stream >> 32) as u32;
            codes
                .binary_search(&(possible_code | (1 << possible_code_len)))
                .map(|idx| {
                    if ret_idx < num_symbols {
                        ret[ret_idx] = symbols[idx];
                        ret_idx += 1;
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
