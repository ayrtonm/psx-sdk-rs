use crate::pub_for_tests;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Alias for a fixed-point number with 12-bit fractional part.
#[allow(non_camel_case_types)]
pub type f16 = FixedPoint<12>;

/// Creates a new fixed-point number with a 12-bit fractional part from a
/// float.
pub const fn f16(x: f32) -> f16 {
    FixedPoint::from_float(x)
}

/// Creates an array of fixed-point numbers with a 12-bit fractional part from an f32 array.
pub const fn f16_array<const N: usize>(ar: [f32; N]) -> [f16; N] {
    let mut i = 0;
    let mut ret = [f16(0.0); N];
    while i < ar.len() {
        ret[i] = f16(ar[i]);
        i += 1;
    }
    ret
}

/// Fixed-point number with `N`-bit fraction
#[repr(C)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct FixedPoint<const N: usize>(i16);
impl<const N: usize> FixedPoint<N> {
    /// Factor to convert between `Self` and `f32`.
    pub const SCALE: usize = 1 << N;
    /// Smallest representable fraction .
    pub const EPSILON: f32 = 1.0 / Self::SCALE as f32;

    /// Creats a new fixed-point number from its bits.
    pub const fn from_bits(x: i16) -> Self {
        Self(x)
    }
    /// Creates a new fixed-point number from a floating point number.
    pub const fn from_float(mut x: f32) -> Self {
        x /= Self::EPSILON;
        Self(x as i16)
    }

    /// Gets the fixed-point number's bits.
    pub const fn to_bits(&self) -> u16 {
        self.0 as u16
    }

    /// Move the fixed-point
    pub const fn upscale(&self) -> i16 {
        self.0
    }

    /// Converts a fixed-point number to a floating point number.
    pub const fn as_f32(&self) -> f32 {
        (self.0 as f32) * Self::EPSILON
    }

    /// Checks if the number is negative.
    pub const fn is_negative(&self) -> bool {
        self.0 < 0
    }

    /// Truncates the fractional part of a fixed-point number.
    pub const fn trunc(&self) -> i16 {
        self.0 >> N
    }

    /// Gets the fractional part of a fixed-point number scaled by `2^N`.
    pub const fn fract(&self) -> i16 {
        self.0 & (Self::SCALE - 1) as i16
    }

    /// Computes the loss in precision when converting from a 32-bit floating
    /// point number.
    pub const fn precision_loss(x: f32) -> f32 {
        x - Self::from_float(x).as_f32()
    }

    pub_for_tests! {
        /// Adds two fixed-point numbers.
        const fn const_add(self, other: Self) -> Self {
            Self(self.0 + other.0)
        }
    }

    pub_for_tests! {
        /// Subtracts two fixed-point numbers.
        const fn const_sub(self, other: Self) -> Self {
            Self(self.0 - other.0)
        }
    }

    pub_for_tests! {
        /// Multiplies two fixed-point numbers.
        const fn const_mul(self, other: Self) -> Self {
            Self(((self.0 as i32 * other.0 as i32) >> N) as i16)
        }
    }
    pub_for_tests! {
        /// Divides two fixed-point numbers.
        const fn const_div(self, other: Self) -> Self {
            Self((((self.0 as i32) << N) / other.0 as i32) as i16)
        }
    }
}

impl From<f32> for f16 {
    fn from(x: f32) -> f16 {
        f16(x)
    }
}

impl<F: Into<f16>> Add<F> for f16 {
    type Output = Self;

    fn add(self, other: F) -> Self::Output {
        self.const_add(other.into())
    }
}

impl Add<f16> for f32 {
    type Output = f16;

    fn add(self, other: f16) -> Self::Output {
        f16::from(self).add(other)
    }
}

impl<F: Into<f16>> AddAssign<F> for f16 {
    fn add_assign(&mut self, other: F) {
        *self = self.add(other.into());
    }
}

impl<F: Into<f16>> Sub<F> for f16 {
    type Output = Self;

    fn sub(self, other: F) -> Self::Output {
        self.const_sub(other.into())
    }
}

impl Sub<f16> for f32 {
    type Output = f16;

    fn sub(self, other: f16) -> Self::Output {
        f16::from(self).sub(other)
    }
}

impl<F: Into<f16>> SubAssign<F> for f16 {
    fn sub_assign(&mut self, other: F) {
        *self = self.sub(other.into());
    }
}

impl<F: Into<f16>> Mul<F> for f16 {
    type Output = Self;

    fn mul(self, other: F) -> Self::Output {
        self.const_mul(other.into())
    }
}

impl Mul<f16> for f32 {
    type Output = f16;

    fn mul(self, other: f16) -> Self::Output {
        f16::from(self).mul(other)
    }
}

impl<F: Into<f16>> MulAssign<F> for f16 {
    fn mul_assign(&mut self, other: F) {
        *self = self.mul(other.into());
    }
}

impl<F: Into<f16>> Div<F> for f16 {
    type Output = Self;

    fn div(self, other: F) -> Self::Output {
        self.const_div(other.into())
    }
}

impl Div<f16> for f32 {
    type Output = f16;

    fn div(self, other: f16) -> Self::Output {
        f16::from(self).div(other)
    }
}

impl<F: Into<f16>> DivAssign<F> for f16 {
    fn div_assign(&mut self, other: F) {
        *self = self.div(other.into());
    }
}

impl Neg for f16 {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}
