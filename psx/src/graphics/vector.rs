use core::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use crate::graphics::trig::{sin, cos};
use crate::graphics::{Vi, f16, Vf};

impl Vi {
    pub const ZERO: Vi = Vi(0, 0);
    pub const X: Vi = Vi(1, 0);
    pub const Y: Vi = Vi(0, 1);
}

impl From<Vi> for u32 {
    fn from(Vi(x, y): Vi) -> u32 {
        x as u32 | (y as u32) << 16
    }
}

impl Vf {
    pub const ZERO: Vf = Vf(f16::ZERO, f16::ZERO, f16::ZERO);
    pub const X: Vf = Vf(f16::ONE, f16::ZERO, f16::ZERO);
    pub const Y: Vf = Vf(f16::ZERO, f16::ONE, f16::ZERO);
    pub const Z: Vf = Vf(f16::ZERO, f16::ZERO, f16::ONE);

    pub fn apply_matrix(self, matrix: [f16; 9]) -> Self {
        let Vf(x, y, z) = self;
        let a = (matrix[0] * x) + (matrix[1] * y) + (matrix[2] * z);
        let b = (matrix[3] * x) + (matrix[4] * y) + (matrix[5] * z);
        let c = (matrix[6] * x) + (matrix[7] * y) + (matrix[8] * z);
        Vf(a, b, c)
    }

    pub fn rotate_x(self, theta: f16, center: Self) -> Self {
        let diff = self - center;
        let M = [
            f16::ONE, f16::ZERO, f16::ZERO,
            f16::ZERO, cos(theta), -sin(theta),
            f16::ZERO, sin(theta), cos(theta),
        ];
        diff.apply_matrix(M) + center
    }

    pub fn rotate_y(self, theta: f16, center: Self) -> Self {
        let diff = self - center;
        let M = [
            cos(theta), f16::ZERO, sin(theta),
            f16::ZERO, f16::ONE, f16::ZERO,
            -sin(theta), f16::ZERO, cos(theta),
        ];
        diff.apply_matrix(M) + center
    }

    pub fn rotate_z(self, theta: f16, center: Self) -> Self {
        let diff = self - center;
        let M = [
            cos(theta), -sin(theta), f16::ZERO,
            sin(theta), cos(theta), f16::ZERO,
            f16::ZERO, f16::ZERO, f16::ONE,
        ];
        diff.apply_matrix(M) + center
    }
}

impl AddAssign<Vi> for Vi {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Add<Vi> for Vi {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl SubAssign<Vi> for Vi {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Sub<Vi> for Vi {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        self -= rhs;
        self
    }
}

impl AddAssign<Vf> for Vf {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Add<Vf> for Vf {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl SubAssign<Vf> for Vf {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl Sub<Vf> for Vf {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        self -= rhs;
        self
    }
}

//impl MulAssign<i16> for Vf {
//    fn mul_assign(&mut self, rhs: i16) {
//        self.0 *= rhs;
//        self.1 *= rhs;
//        self.2 *= rhs;
//    }
//}

//impl Mul<i16> for Vf {
//    type Output = Self;
//    fn mul(mut self, rhs: i16) -> Self {
//        self *= rhs;
//        self
//    }
//}

impl MulAssign<f16> for Vf {
    fn mul_assign(&mut self, rhs: f16) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Mul<f16> for Vf {
    type Output = Self;
    fn mul(mut self, rhs: f16) -> Self {
        self *= rhs;
        self
    }
}

impl DivAssign<i16> for Vf {
    fn div_assign(&mut self, rhs: i16) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl Div<i16> for Vf {
    type Output = Self;
    fn div(mut self, rhs: i16) -> Self {
        self /= rhs;
        self
    }
}

impl DivAssign<f16> for Vf {
    fn div_assign(&mut self, rhs: f16) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl Div<f16> for Vf {
    type Output = Self;
    fn div(mut self, rhs: f16) -> Self {
        self /= rhs;
        self
    }
}
