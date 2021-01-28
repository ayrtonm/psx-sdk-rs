pub const fn format_u32(mut x: u32, leading_zeros: bool, hexdecimal: bool) -> [u8; 10] {
    let mut leading = !leading_zeros;
    let mut ar = [0; 10];
    let mut current_digit = 0;
    let max_digits = if hexdecimal { 8 } else { 10 };
    const_for! {
        i in 0, max_digits => {
            let digit = if hexdecimal {
                (x >> ((7 - i) * 4)) & 0xF
            } else {
                let digit = x / (10u32.pow(9 - i));
                x -= digit * 10u32.pow(9 - i);
                digit
            };
            if digit != 0 || i == max_digits - 1 {
                leading = false;
            };
            if !leading {
                let digit = digit as u8;
                ar[current_digit] = if digit  < 10 {
                    b'0' + digit
                } else {
                    b'A' + digit - 10
                };
                current_digit += 1;
            }
        }
    }
    if hexdecimal {
        ar[current_digit] = b'h';
    }
    ar
}

#[cfg(test)]
mod tests {
    use super::format_u32;

    #[allow(dead_code)]
    const DEC_FMT: () = {
        let hex = false;
        let leading = false;
        assert!(slice_cmp!(
            &format_u32(29, leading, hex),
            b"29\0\0\0\0\0\0\0\0"
        ));
        assert!(slice_cmp!(
            &format_u32(0xFFFF_FFFF, leading, hex),
            b"4294967295"
        ));
        assert!(slice_cmp!(
            &format_u32(0, leading, hex),
            b"0\0\0\0\0\0\0\0\0\0"
        ));
    };

    #[allow(dead_code)]
    const HEX_FMT: () = {
        let hex = true;
        let leading = false;
        assert!(slice_cmp!(
            &format_u32(29, leading, hex),
            b"1Dh\0\0\0\0\0\0\0"
        ));
        assert!(slice_cmp!(
            &format_u32(0xFFFF_FFFF, leading, hex),
            b"FFFFFFFFh\0"
        ));
        assert!(slice_cmp!(
            &format_u32(0, leading, hex),
            b"0h\0\0\0\0\0\0\0\0"
        ));
    };

    #[allow(dead_code)]
    const DEC_ZERO_FMT: () = {
        let hex = false;
        let leading = true;
        assert!(slice_cmp!(&format_u32(29, leading, hex), b"0000000029"));
        assert!(slice_cmp!(
            &format_u32(0xFFFF_FFFF, leading, hex),
            b"4294967295"
        ));
        assert!(slice_cmp!(&format_u32(0, leading, hex), b"0000000000"));
    };

    #[allow(dead_code)]
    const HEX_ZERO_FMT: () = {
        let hex = true;
        let leading = true;
        assert!(slice_cmp!(&format_u32(29, leading, hex), b"0000001Dh\0"));
        assert!(slice_cmp!(
            &format_u32(0xFFFF_FFFF, leading, hex),
            b"FFFFFFFFh\0"
        ));
        assert!(slice_cmp!(&format_u32(0, leading, hex), b"00000000h\0"));
    };
}
