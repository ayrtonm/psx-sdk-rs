use crate::graphics::f16;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A signed 16-bit fixed point number with `FRAC` fractional bits.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct F16<const FRAC: usize>(pub i16);

impl<const FRAC: usize> F16<FRAC> {
    pub const ZERO: Self = Self(0x0_000);
    pub const ONE: Self = Self(1 << FRAC);
    pub const INT_MASK: i16 = ((1 << (16 - FRAC - 1)) - 1) << FRAC;
    pub const FRAC_MASK: i16 = (1 << FRAC) - 1;
    pub const SIGN_MASK: i16 = 0x8000u16 as i16;

    // Raw transmutation from `u16`.
    pub fn from_bits(bits: u16) -> Self {
        Self(bits as i16)
    }

    /// Raw transmutation to `u16`.
    pub fn to_bits(&self) -> u16 {
        self.0 as u16
    }

    // Computes the absolute value of `self`.
    pub fn abs(self) -> Self {
        let mask = Self::INT_MASK | Self::FRAC_MASK;
        Self(self.0 & mask)
    }

    // Returns the integer part of a number.
    pub fn trunc(self) -> Self {
        let mask = Self::INT_MASK | Self::SIGN_MASK;
        Self(self.0 & mask)
    }

    // Returns the fractional part of a number.
    pub fn fract(self) -> Self {
        let mask = Self::FRAC_MASK | Self::SIGN_MASK;
        Self(self.0 & mask)
    }

    pub fn is_negative(&self) -> bool {
        (self.0 >> 15) != 0
    }

    pub fn is_positive(&self) -> bool {
        !self.is_negative()
    }

    // Returns a number that represents the sign of `self`.
    pub fn signum(self) -> Self {
        if self.0 == 0 {
            Self::ZERO
        } else if self.is_negative() {
            -Self::ONE
        } else {
            Self::ONE
        }
    }

    // Potentially lossy conversion from an integer. Result may overflow.
    pub fn lossy_from_int(x: i16) -> Self {
        let sign = if x.is_negative() { Self::SIGN_MASK } else { 0 };
        let modulus = 1 << (16 - FRAC - 1);
        let int = (x.abs() % modulus) << FRAC;
        Self(sign | int)
    }

    // Potentially lossy conversion to an integer. Result may be truncated.
    pub fn lossy_to_int(self) -> i16 {
        let sign = if self.is_negative() { -1i16 } else { 1i16 };
        let int = self.abs().trunc().0 >> FRAC;
        sign * int
    }

    pub fn wrapping_div(lhs: i16, rhs: i16) -> Self {
        let sign = if lhs.is_negative() != rhs.is_negative() {
            Self::SIGN_MASK
        } else {
            0
        };
        let lhs = lhs.abs() as u16 as u64;
        let rhs = rhs.abs() as u16 as u64;
        let res = (lhs << (16 + FRAC)) / (rhs << 16);
        Self((res as u16 as i16) | sign)
    }
}

impl<const FRAC: usize> From<F16<FRAC>> for f32 {
    fn from(x: F16<FRAC>) -> Self {
        let sign = if x.is_negative() { -1.0 } else { 1.0 };
        let int = Self::from(x.abs().lossy_to_int());
        let frac = Self::from(x.abs().fract().0) / 4096.0;
        (int + frac) * sign
    }
}

impl<const FRAC: usize> From<F16<FRAC>> for f64 {
    fn from(x: F16<FRAC>) -> Self {
        let sign = if x.is_negative() { -1.0 } else { 1.0 };
        let int = Self::from(x.abs().lossy_to_int());
        let frac = Self::from(x.abs().fract().0) / 4096.0;
        (int + frac) * sign
    }
}

impl<const FRAC: usize> Neg for F16<FRAC> {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
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

impl<const FRAC: usize> Mul<i16> for F16<FRAC> {
    type Output = i16;
    fn mul(self, rhs: i16) -> i16 {
        let sign = if self.is_negative() != rhs.is_negative() {
            -1i16
        } else {
            1i16
        };
        let lhs = self.abs().0 as u16 as u32;
        let rhs = rhs.abs() as u16 as u32;
        let res = (lhs * rhs) >> FRAC;
        (res as u16 as i16) * sign
    }
}

impl<const FRAC: usize> Mul<F16<FRAC>> for i16 {
    type Output = i16;
    fn mul(self, rhs: F16<FRAC>) -> i16 {
        rhs * self
    }
}

impl<const FRAC: usize> MulAssign<F16<FRAC>> for i16 {
    fn mul_assign(&mut self, rhs: F16<FRAC>) {
        *self = *self * rhs;
    }
}

impl<const FRAC: usize> Mul<F16<FRAC>> for F16<FRAC> {
    type Output = i16;
    fn mul(self, rhs: F16<FRAC>) -> i16 {
        let sign = if self.is_negative() != rhs.is_negative() {
            -1i16
        } else {
            1i16
        };
        let lhs = self.abs().0 as u16 as u32;
        let rhs = rhs.abs().0 as u16 as u32;
        let res = (lhs * rhs) >> (2 * FRAC);
        (res as u16 as i16) * sign
    }
}

impl<const FRAC: usize> Div<F16<FRAC>> for F16<FRAC> {
    type Output = i16;
    fn div(self, rhs: Self) -> i16 {
        let sign = if self.is_negative() != rhs.is_negative() {
            -1i16
        } else {
            1i16
        };
        let lhs = self.abs().0 as u16 as u32;
        let rhs = rhs.abs().0 as u16 as u32;
        let res = (lhs << 16) / (rhs << 16);
        (res as u16 as i16) * sign
    }
}

impl<const FRAC: usize> Div<F16<FRAC>> for i16 {
    type Output = Self;
    fn div(self, rhs: F16<FRAC>) -> Self {
        let sign = if self.is_negative() != rhs.is_negative() {
            -1i16
        } else {
            1i16
        };
        let lhs = self.abs() as u16 as u32;
        let rhs = rhs.abs().0 as u16 as u32;
        let res = ((lhs << 16) / rhs) >> 16;
        (res as u16 as i16) * sign
    }
}

impl<const FRAC: usize> DivAssign<F16<FRAC>> for i16 {
    fn div_assign(&mut self, rhs: F16<FRAC>) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::f16;

    fn test_approx<F, G, R>(exact: F, approx: G, (low, high): (R, R), threshold: f32)
    where
        R: Copy + core::fmt::Debug,
        F: Fn(i16, i16) -> R,
        G: Fn(i16, i16) -> f32,
        f32: From<R>, {
        fuzz!(|x: i16, y: i16| {
            let reference = approx(x, y);
            if reference < f32::from(high) && reference > f32::from(low) {
                let res = exact(x, y);
                let diff = f32::from(res) - reference;
                let cond = diff < threshold && diff > -threshold;
                if !cond {
                    crate::println!("{:?} {:?}", res, reference);
                }
                assert!(cond);
            }
        });
    }

    #[test_case]
    fn mul() {
        let float_mul = |x, y| f32::from(f16(x)) * f32::from(f16(y));
        let fixed_mul = |x, y| f16(x) * f16(y);
        test_approx(fixed_mul, float_mul, (i16::MIN, i16::MAX), 1.0);
    }

    #[test_case]
    fn wrapping_mul() {
        let float_mul = |x, y| f32::from(f16(x)) * f32::from(y);
        let fixed_mul = |x, y| f16(x) * y;
        test_approx(fixed_mul, float_mul, (i16::MIN, i16::MAX), 1.0);
    }

    #[test_case]
    fn wrapping_div() {
        let float_div = |x, y| f32::from(x) / f32::from(y);
        let fixed_div = |x, y| f16::wrapping_div(x, y);
        test_approx(fixed_div, float_div, (f16(-0x7_FFF), f16(0x7_FFF)), 1.0);
    }

    #[test_case]
    fn div() {
        let float_div = |x, y| f32::from(f16(x)) / f32::from(f16(y));
        let fixed_div = |x, y| f16(x) / f16(y);
        test_approx(fixed_div, float_div, (i16::MIN, i16::MAX), 1.0);
    }

    #[test_case]
    fn div2() {
        let float_div = |x, y| f32::from(x) / (f32::from(f16(y)) * 4096.0);
        let fixed_div = |x, y| x / f16(y);
        test_approx(fixed_div, float_div, (i16::MIN, i16::MAX), 1.0);
    }

    #[test_case]
    fn neg() {
        fuzz!(|x: i16| {
            let neg_x = -f16(x);
            assert!(neg_x.0 == -x);
        });
    }

    #[test_case]
    fn add() {
        fuzz!(|x: i16, y: i16| {
            let fz = f16(x) + f16(y);
            let z = x + y;
            assert!(fz.0 == z);
        });
    }

    #[test_case]
    fn sub() {
        fuzz!(|x: i16, y: i16| {
            let fz = f16(x) - f16(y);
            let z = x - y;
            assert!(fz.0 == z);
        });
    }

    #[test_case]
    fn add_assign() {
        fuzz!(|x: i16, y: i16| {
            let mut fz = f16(x);
            fz += f16(y);
            let z = x + y;
            assert!(fz.0 == z);
        });
    }

    #[test_case]
    fn sub_assign() {
        fuzz!(|x: i16, y: i16| {
            let mut fz = f16(x);
            fz -= f16(y);
            let z = x - y;
            assert!(fz.0 == z);
        });
    }

    #[test_case]
    fn lossless_int_conversion() {
        fuzz!(|x: i16| {
            // Ensure all conversions are lossless
            let mod_x = x % 8;
            assert!(f16::lossy_from_int(mod_x).lossy_to_int() == mod_x);
        });
    }
}
