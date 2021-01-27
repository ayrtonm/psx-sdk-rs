pub const fn format_u32(x: u32, leading_zeros: bool, hexdecimal: bool) -> [u8; 10] {
    const fn to_ascii(x: u8) -> u8 {
        if x < 10 {
            b'0' + x
        } else {
            b'A' + x - 10
        }
    }
    let mut leading = !leading_zeros;
    let mut ar = [0; 10];
    let mut j = 0;
    let max_digits = if hexdecimal { 8 } else { 10 };
    let mut y = x;
    let mut i = 0;
    while i < max_digits {
        let digit = if hexdecimal {
            (x >> ((7 - i) * 4)) & 0xF
        } else {
            let digit = y / (10u32.pow(9 - i));
            y -= digit * 10u32.pow(9 - i);
            digit
        };
        if digit != 0 || i == max_digits - 1 {
            leading = false;
        };
        if !leading {
            let as_char = to_ascii(digit as u8);
            ar[j] = as_char;
            j += 1;
        }
        i += 1;
    }
    if hexdecimal {
        ar[j] = b'h';
    }
    ar
}

test! {
    const fn dec_fmt() {
        let hex = false;
        let leading = false;
        assert!(slice_cmp!(&format_u32(29, leading, hex), b"29\0\0\0\0\0\0\0\0"));
        assert!(slice_cmp!(&format_u32(0xFFFF_FFFF, leading, hex), b"4294967295"));
        assert!(slice_cmp!(&format_u32(0, leading, hex), b"0\0\0\0\0\0\0\0\0\0"));
        ok!()
    }
}

test! {
    const fn hex_fmt() {
        let hex = true;
        let leading = false;
        assert!(slice_cmp!(&format_u32(29, leading, hex), b"1Dh\0\0\0\0\0\0\0"));
        assert!(slice_cmp!(&format_u32(0xFFFF_FFFF, leading, hex), b"FFFFFFFFh\0"));
        assert!(slice_cmp!(&format_u32(0, leading, hex), b"0h\0\0\0\0\0\0\0\0"));
        ok!()
    }
}

test! {
    const fn dec_zero_fmt() {
        let hex = false;
        let leading = true;
        assert!(slice_cmp!(&format_u32(29, leading, hex), b"0000000029"));
        assert!(slice_cmp!(&format_u32(0xFFFF_FFFF, leading, hex), b"4294967295"));
        assert!(slice_cmp!(&format_u32(0, leading, hex), b"0000000000"));
        ok!()
    }
}

test! {
    const fn hex_zero_fmt() {
        let hex = true;
        let leading = true;
        assert!(slice_cmp!(&format_u32(29, leading, hex), b"0000001Dh\0"));
        assert!(slice_cmp!(&format_u32(0xFFFF_FFFF, leading, hex), b"FFFFFFFFh\0"));
        assert!(slice_cmp!(&format_u32(0, leading, hex), b"00000000h\0"));
        ok!()
    }
}
