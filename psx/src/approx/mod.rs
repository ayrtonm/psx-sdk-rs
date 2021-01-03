use core::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

/// Approximations for trigonometric functions.
pub mod trig;

/// Alias for a fixed-point number with 12-bit fractional part.
#[allow(non_camel_case_types)]
pub type f16 = FixedPoint<12>;

//impl From<f32> for f16 {
//    fn from(x: f32) -> f16 {
//        FixedPoint::new(x)
//    }
//}

impl f16 {
    /// Creates a new fixed-point number with a 12-bit fractional part from a
    /// float.
    pub const fn new(x: f32) -> Self {
        Self::from_float(x)
    }
}

/// Fixed-point number with `N`-bit fraction
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct FixedPoint<const N: usize>(i16);
impl<const N: usize> FixedPoint<N> {
    /// Factor to convert between `Self` and `f32`.
    pub const SCALE: f32 = (1 << N) as f32;

    /// Creates a new fixed-point number from a floating point number.
    pub const fn from_float(mut x: f32) -> Self {
        x *= Self::SCALE;
        Self(x as i16)
    }

    /// Gets the fixed-point number's bits.
    pub const fn to_bits(&self) -> u16 {
        self.0 as u16
    }

    /// Converts a fixed-point number to a floating point number.
    pub const fn as_float(&self) -> f32 {
        (self.0 as f32) / Self::SCALE
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
        self.0 & (Self::SCALE - 1.0) as i16
    }

    /// Computes the loss in precision when converting from a 32-bit floating
    /// point number.
    pub const fn precision_loss(x: f32) -> f32 {
        x - Self::from_float(x).as_float()
    }

    /// Adds two fixed-point numbers.
    pub const fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }

    /// Subtracts two fixed-point numbers.
    pub const fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }

    /// Multiplies two fixed-point numbers.
    pub const fn mul(self, _other: Self) -> Self {
        Self(0)
        //Self(((self.0 as i32 * other.0 as i32) >> (2 * N)) as i16)
        //Self(((self.0 as i32 * other.0 as i32) >> N) as i16)
        //Self((self.0 * other.0) >> (2 * N))
        //Self((self.0 * other.0) >> N)
    }
}

impl Add<f16> for f16 {
    type Output = f16;

    fn add(self, other: Self) -> Self {
        self.add(other)
    }
}

impl AddAssign<f16> for f16 {
    fn add_assign(&mut self, other: Self) {
        *self = self.add(other);
    }
}

impl Sub<f16> for f16 {
    type Output = f16;

    fn sub(self, other: Self) -> Self {
        self.sub(other)
    }
}

impl SubAssign<f16> for f16 {
    fn sub_assign(&mut self, other: Self) {
        *self = self.sub(other);
    }
}

impl Mul<f16> for f16 {
    type Output = f16;

    fn mul(self, other: Self) -> Self {
        self.mul(other)
    }
}

impl MulAssign<f16> for f16 {
    fn mul_assign(&mut self, other: Self) {
        *self = self.mul(other);
    }
}
