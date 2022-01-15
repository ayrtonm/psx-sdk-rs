use crate::graphics::{cos, f16, sin, Vf, Vi};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

impl Vi {
    pub const ZERO: Vi = Vi(0, 0, 0);
    pub const X: Vi = Vi(1, 0, 0);
    pub const Y: Vi = Vi(0, 1, 0);
    pub const Z: Vi = Vi(0, 0, 1);

    pub fn apply_matrix(self, matrix: [f16; 9]) -> Self {
        let Self(x, y, z) = self;
        let a = (matrix[0] * x) + (matrix[1] * y) + (matrix[2] * z);
        let b = (matrix[3] * x) + (matrix[4] * y) + (matrix[5] * z);
        let c = (matrix[6] * x) + (matrix[7] * y) + (matrix[8] * z);
        Self(a, b, c)
    }

    pub fn rotate_x(self, theta: f16, center: Self) -> Self {
        let diff = self - center;
        let m = [
            f16::ONE,
            f16::ZERO,
            f16::ZERO,
            f16::ZERO,
            cos(theta),
            -sin(theta),
            f16::ZERO,
            sin(theta),
            cos(theta),
        ];
        diff.apply_matrix(m) + center
    }

    pub fn rotate_y(self, theta: f16, center: Self) -> Self {
        let diff = self - center;
        let m = [
            cos(theta),
            f16::ZERO,
            sin(theta),
            f16::ZERO,
            f16::ONE,
            f16::ZERO,
            -sin(theta),
            f16::ZERO,
            cos(theta),
        ];
        diff.apply_matrix(m) + center
    }

    pub fn rotate_z(self, theta: f16, center: Self) -> Self {
        let diff = self - center;
        let m = [
            cos(theta),
            -sin(theta),
            f16::ZERO,
            sin(theta),
            cos(theta),
            f16::ZERO,
            f16::ZERO,
            f16::ZERO,
            f16::ONE,
        ];
        diff.apply_matrix(m) + center
    }
}

impl Add<Vi> for Vi {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
        self
    }
}

impl AddAssign<Vi> for Vi {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub<Vi> for Vi {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
        self
    }
}

impl SubAssign<Vi> for Vi {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<Vi> for i16 {
    type Output = Vi;
    fn mul(self, rhs: Vi) -> Vi {
        rhs.mul(self)
    }
}

impl Mul<i16> for Vi {
    type Output = Vi;
    fn mul(mut self, rhs: i16) -> Vi {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
        self
    }
}

impl MulAssign<i16> for Vi {
    fn mul_assign(&mut self, rhs: i16) {
        *self = *self * rhs;
    }
}

impl Div<i16> for Vi {
    type Output = Vi;
    fn div(mut self, den: i16) -> Vi {
        self.0 /= den;
        self.1 /= den;
        self.2 /= den;
        self
    }
}

impl DivAssign<i16> for Vi {
    fn div_assign(&mut self, den: i16) {
        *self = *self / den;
    }
}

impl Vf {
    pub const ZERO: Vf = Vf(f16::ZERO, f16::ZERO, f16::ZERO);
    pub const X: Vf = Vf(f16::ONE, f16::ZERO, f16::ZERO);
    pub const Y: Vf = Vf(f16::ZERO, f16::ONE, f16::ZERO);
    pub const Z: Vf = Vf(f16::ZERO, f16::ZERO, f16::ONE);

    pub fn apply_matrix(self, matrix: [f16; 9]) -> Self {
        let Self(x, y, z) = self;
        let a = (matrix[0] * x) + (matrix[1] * y) + (matrix[2] * z);
        let b = (matrix[3] * x) + (matrix[4] * y) + (matrix[5] * z);
        let c = (matrix[6] * x) + (matrix[7] * y) + (matrix[8] * z);
        Self(a, b, c)
    }

    pub fn rotate_x(self, theta: f16, center: Self) -> Self {
        let diff = self - center;
        let m = [
            f16::ONE,
            f16::ZERO,
            f16::ZERO,
            f16::ZERO,
            cos(theta),
            -sin(theta),
            f16::ZERO,
            sin(theta),
            cos(theta),
        ];
        diff.apply_matrix(m) + center
    }

    pub fn rotate_y(self, theta: f16, center: Self) -> Self {
        let diff = self - center;
        let m = [
            cos(theta),
            f16::ZERO,
            sin(theta),
            f16::ZERO,
            f16::ONE,
            f16::ZERO,
            -sin(theta),
            f16::ZERO,
            cos(theta),
        ];
        diff.apply_matrix(m) + center
    }

    pub fn rotate_z(self, theta: f16, center: Self) -> Self {
        let diff = self - center;
        let m = [
            cos(theta),
            -sin(theta),
            f16::ZERO,
            sin(theta),
            cos(theta),
            f16::ZERO,
            f16::ZERO,
            f16::ZERO,
            f16::ONE,
        ];
        diff.apply_matrix(m) + center
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

//impl MulAssign<f16> for Vf {
//    fn mul_assign(&mut self, rhs: f16) {
//        self.0 *= rhs;
//        self.1 *= rhs;
//        self.2 *= rhs;
//    }
//}
//
//impl Mul<f16> for Vf {
//    type Output = Self;
//    fn mul(mut self, rhs: f16) -> Self {
//        self *= rhs;
//        self
//    }
//}
//
//impl DivAssign<i16> for Vf {
//    fn div_assign(&mut self, rhs: i16) {
//        self.0 /= rhs;
//        self.1 /= rhs;
//        self.2 /= rhs;
//    }
//}
//
//impl Div<i16> for Vf {
//    type Output = Self;
//    fn div(mut self, rhs: i16) -> Self {
//        self /= rhs;
//        self
//    }
//}
//
//impl DivAssign<f16> for Vf {
//    fn div_assign(&mut self, rhs: f16) {
//        self.0 /= rhs;
//        self.1 /= rhs;
//        self.2 /= rhs;
//    }
//}
//
//impl Div<f16> for Vf {
//    type Output = Self;
//    fn div(mut self, rhs: f16) -> Self {
//        self /= rhs;
//        self
//    }
//}
