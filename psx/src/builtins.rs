#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if src < dest as *const u8 {
        // copy from end
        let mut i = n;
        while i != 0 {
            i -= 1;
            *dest.offset(i as isize) = *src.offset(i as isize);
        }
    } else {
        // copy from beginning
        let mut i = 0;
        while i < n {
            *dest.offset(i as isize) = *src.offset(i as isize);
            i += 1;
        }
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = c as u8;
        i += 1;
    }
    s
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.offset(i as isize);
        let b = *s2.offset(i as isize);
        if a != b {
            return a as i32 - b as i32
        }
        i += 1;
    }
    0
}

#[no_mangle]
pub fn fmodf(x: f32, y: f32) -> f32 {
    let mut uxi = x.to_bits();
    let mut uyi = y.to_bits();
    let mut ex = (uxi >> 23 & 0xff) as i32;
    let mut ey = (uyi >> 23 & 0xff) as i32;
    let sx = uxi & 0x80000000;
    let mut i;

    if uyi << 1 == 0 || y.is_nan() || ex == 0xff {
        return (x * y) / (x * y)
    }

    if uxi << 1 <= uyi << 1 {
        if uxi << 1 == uyi << 1 {
            return 0.0 * x
        }

        return x
    }

    /* normalize x and y */
    if ex == 0 {
        i = uxi << 9;
        while i >> 31 == 0 {
            ex -= 1;
            i <<= 1;
        }

        uxi <<= -ex + 1;
    } else {
        uxi &= u32::MAX >> 9;
        uxi |= 1 << 23;
    }

    if ey == 0 {
        i = uyi << 9;
        while i >> 31 == 0 {
            ey -= 1;
            i <<= 1;
        }

        uyi <<= -ey + 1;
    } else {
        uyi &= u32::MAX >> 9;
        uyi |= 1 << 23;
    }

    /* x mod y */
    while ex > ey {
        i = uxi.wrapping_sub(uyi);
        if i >> 31 == 0 {
            if i == 0 {
                return 0.0 * x
            }
            uxi = i;
        }
        uxi <<= 1;

        ex -= 1;
    }

    i = uxi.wrapping_sub(uyi);
    if i >> 31 == 0 {
        if i == 0 {
            return 0.0 * x
        }
        uxi = i;
    }

    while uxi >> 23 == 0 {
        uxi <<= 1;
        ex -= 1;
    }

    /* scale result up */
    if ex > 0 {
        uxi -= 1 << 23;
        uxi |= (ex as u32) << 23;
    } else {
        uxi >>= -ex + 1;
    }
    uxi |= sx;

    f32::from_bits(uxi)
}
