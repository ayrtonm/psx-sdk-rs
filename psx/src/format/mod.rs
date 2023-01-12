//! Support for parsing various file formats

use crate::math::f16;

pub mod mtl;
pub mod obj;
pub mod tim;

/// Parse an `f16` from a byte slice starting at `idx`.
#[doc(hidden)]
pub const fn parse_f16(data: &[u8], idx: &mut usize) -> f16 {
    let neg = data[*idx] == b'-';
    if neg {
        *idx += 1;
    }
    let abs_int = (data[*idx] - b'0') as u16;
    assert!(abs_int < 2u16.pow(f16::INT as u32));
    *idx += 1;
    assert!(data[*idx] == b'.');
    *idx += 1;
    let mut frac = 0;
    let mut digits = 0;
    while data[*idx] != b' ' && data[*idx] != b'\n' {
        frac *= 10;
        frac += (data[*idx] - b'0') as u64;
        digits += 1;
        *idx += 1;
    }
    *idx += 1;
    let abs_frac = (frac * 2u64.pow(f16::FRAC as u32) / 10u64.pow(digits)) as u16;
    let abs_fixed = (abs_int << f16::FRAC) | abs_frac;
    let fixed = if neg {
        -(abs_fixed as i16)
    } else {
        abs_fixed as i16
    };
    f16(fixed)
}
