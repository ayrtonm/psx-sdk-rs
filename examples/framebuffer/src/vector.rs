use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use paste::paste;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct V2(pub i16, pub i16);

pub const ZERO2: V2 = V2(0, 0);
pub const X2: V2 = V2(1, 0);
pub const Y2: V2 = V2(0, 1);

impl From<V2> for [i16; 2] {
    fn from(v: V2) -> [i16; 2] {
        [v.0, v.1]
    }
}

macro_rules! impl_op_v2 {
    ($fn:ident, $op:tt) => {
        paste! {
            impl [<$fn:camel>]<i16> for V2 {
                type Output = Self;
                fn $fn(mut self, rhs: i16) -> Self {
                    self $op rhs;
                    self
                }
            }
            impl [<$fn:camel>]<Self> for V2 {
                type Output = Self;
                fn $fn(mut self, rhs: Self) -> Self {
                    self $op rhs;
                    self
                }
            }
            impl [<$fn:camel Assign>]<i16> for V2 {
                fn [<$fn _assign>](&mut self, rhs: i16) {
                    self.0 $op rhs;
                    self.1 $op rhs;
                }
            }
            impl [<$fn:camel Assign>]<Self> for V2 {
                fn [<$fn _assign>](&mut self, rhs: Self) {
                    self.0 $op rhs.0;
                    self.1 $op rhs.1;
                }
            }
        }
    };
}
impl_op_v2!(add, +=);
impl_op_v2!(sub, -=);
impl_op_v2!(mul, *=);
impl_op_v2!(div, /=);
impl_op_v2!(rem, %=);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct V3(pub i16, pub i16, pub i16);

pub const ZERO: V3 = V3(0, 0, 0);
pub const X: V3 = V3(1, 0, 0);
pub const Y: V3 = V3(0, 1, 0);
pub const Z: V3 = V3(0, 0, 1);

impl From<V3> for [i16; 3] {
    fn from(v: V3) -> [i16; 3] {
        [v.0, v.1, v.2]
    }
}

macro_rules! impl_op_v3 {
    ($fn:ident, $op:tt) => {
        paste! {
            impl [<$fn:camel>]<i16> for V3 {
                type Output = Self;
                fn $fn(mut self, rhs: i16) -> Self {
                    self $op rhs;
                    self
                }
            }
            impl [<$fn:camel>]<Self> for V3 {
                type Output = Self;
                fn $fn(mut self, rhs: Self) -> Self {
                    self $op rhs;
                    self
                }
            }
            impl [<$fn:camel Assign>]<i16> for V3 {
                fn [<$fn _assign>](&mut self, rhs: i16) {
                    self.0 $op rhs;
                    self.1 $op rhs;
                    self.2 $op rhs;
                }
            }
            impl [<$fn:camel Assign>]<Self> for V3 {
                fn [<$fn _assign>](&mut self, rhs: Self) {
                    self.0 $op rhs.0;
                    self.1 $op rhs.1;
                    self.2 $op rhs.2;
                }
            }
        }
    };
}

impl_op_v3!(add, +=);
impl_op_v3!(sub, -=);
impl_op_v3!(mul, *=);
impl_op_v3!(div, /=);
impl_op_v3!(rem, %=);
