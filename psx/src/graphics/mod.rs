#![allow(non_camel_case_types)]

use crate::graphics::fixed_point::F16;

mod cosine_table;
pub mod fixed_point;
pub mod vector;

use cosine_table::COSINE_TABLE;

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

pub fn cos(x: f16) -> f16 {
    let idx = x.to_bits() as u16 as usize;
    f16(COSINE_TABLE[idx])
}

pub fn sin(x: f16) -> f16 {
    cos(f16(0x4000) - x)
}
