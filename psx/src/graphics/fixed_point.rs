use crate::graphics::f16;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A signed 16-bit fixed point number with `FRAC` fractional bits.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct F16<const FRAC: usize>(pub i16);

impl f16 {
    pub const ZERO: f16 = f16(0x0_000);
    pub const ONE: f16 = f16(0x1_000);

    pub fn to_bits(&self) -> i16 {
        self.0
    }
}

impl<const FRAC: usize> From<i16> for F16<FRAC> {
    fn from(x: i16) -> Self {
        Self(x)
    }
}

impl<const FRAC: usize> From<F16<FRAC>> for i16 {
    fn from(x: F16<FRAC>) -> Self {
        x.0 >> FRAC
    }
}

impl<const FRAC: usize> From<F16<FRAC>> for i32 {
    fn from(x: F16<FRAC>) -> Self {
        i32::from(i16::from(x))
    }
}

// TODO: Remove all soft-float
impl<const FRAC: usize> From<f32> for F16<FRAC> {
    fn from(x: f32) -> Self {
        let scale = (1 << FRAC) as f32;
        let res = unsafe { (x * scale).to_int_unchecked::<i16>() };
        Self(res)
    }
}
impl<const FRAC: usize> From<F16<FRAC>> for f32 {
    fn from(x: F16<FRAC>) -> Self {
        (x.0 as f32) / ((1 << FRAC) as f32)
    }
}

impl<const FRAC: usize> Add<F16<FRAC>> for F16<FRAC> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl<const FRAC: usize> AddAssign<F16<FRAC>> for F16<FRAC> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const FRAC: usize> Sub<F16<FRAC>> for F16<FRAC> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl<const FRAC: usize> SubAssign<F16<FRAC>> for F16<FRAC> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const FRAC: usize> Mul<F16<FRAC>> for F16<FRAC> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let lhs = i32::from(self.0);
        let rhs = i32::from(rhs.0);
        let res = (lhs * rhs) >> FRAC;
        Self(res as i16)
    }
}

impl<const FRAC: usize> MulAssign<F16<FRAC>> for F16<FRAC> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const FRAC: usize> Mul<i16> for F16<FRAC> {
    type Output = i16;
    fn mul(self, rhs: i16) -> i16 {
        let lhs = i32::from(self.0);
        let rhs = i32::from(rhs);
        let res = (lhs * rhs) >> FRAC;
        res as i16
    }
}

impl<const FRAC: usize> Mul<F16<FRAC>> for i16 {
    type Output = i16;
    fn mul(self, rhs: F16<FRAC>) -> i16 {
        let lhs = i32::from(self);
        let rhs = i32::from(rhs.0);
        let res = (lhs * rhs) >> FRAC;
        res as i16
    }
}

impl<const FRAC: usize> MulAssign<F16<FRAC>> for i16 {
    fn mul_assign(&mut self, rhs: F16<FRAC>) {
        *self = *self * rhs;
    }
}

impl<const FRAC: usize> Div<F16<FRAC>> for F16<FRAC> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let lhs = i32::from(self.0);
        let rhs = i32::from(rhs.0);
        let res = (lhs << FRAC) / rhs;
        Self(res as i16)
    }
}

impl<const FRAC: usize> DivAssign<F16<FRAC>> for F16<FRAC> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const FRAC: usize> Div<i16> for F16<FRAC> {
    type Output = Self;
    fn div(self, rhs: i16) -> Self {
        let lhs = i32::from(self.0);
        let rhs = i32::from(rhs);
        let res = lhs / rhs;
        Self(res as i16)
    }
}

impl<const FRAC: usize> DivAssign<i16> for F16<FRAC> {
    fn div_assign(&mut self, rhs: i16) {
        *self = *self / rhs;
    }
}

impl<const FRAC: usize> Neg for F16<FRAC> {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}
