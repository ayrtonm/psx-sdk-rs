#![allow(non_camel_case_types)]

use crate::graphics::fixed_point::F16;

pub mod fixed_point;
mod trig;
pub mod vector;

use trig::{COSINE_TABLE, COSINE_TABLE_SIZE};

pub type f16 = F16<12>;

/// Reinterprets an i16 as a signed 16-bit fixed point number with a 12-bit
/// fraction.
///
/// This is the default 16-bit fixed point format for the GTE. The argument can
/// be formatted as `+/-0xA_BCD` where `0xBCD` is the fractional part and `0xA <
/// 0x8` is the integral part. For example `f16(0x_800)` represents `1/2` while
/// `f16(-0x1_800)` represents `-1 1/2`.
pub const fn f16(x: i16) -> f16 {
    F16(x)
}

/// A vector of i16.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vi(pub i16, pub i16, pub i16);

/// A vector of f16.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vf(pub f16, pub f16, pub f16);

pub const PI: f16 = f16(0x8000u16 as i16);
pub const FRAC_PI_2: f16 = f16(0x4000);
pub const FRAC_PI_3: f16 = f16(0x2aaa);
pub const FRAC_PI_4: f16 = f16(0x2000);
pub const FRAC_PI_6: f16 = f16(0x1555);
pub const FRAC_PI_8: f16 = f16(0x1000);

pub fn cos(x: f16) -> f16 {
    let x = x.to_bits() as u16;
    let table_size = COSINE_TABLE_SIZE as u16;
    let quarter_cycle = x / table_size;
    let offset = x % table_size;
    let idx = offset as usize;
    let reverse_idx = (table_size - offset - 1) as usize;
    match quarter_cycle {
        0 => COSINE_TABLE[reverse_idx],
        1 => COSINE_TABLE[idx],
        2 => -COSINE_TABLE[reverse_idx],
        3 => -COSINE_TABLE[idx],
        _ => unreachable!("a u16 divided by 0x4000 can't be greater than 3"),
    }
}

pub fn sin(x: f16) -> f16 {
    cos(FRAC_PI_2 - x)
}
