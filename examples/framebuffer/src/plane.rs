use crate::vector::{V2, V3, X, X2, Y, Y2, Z, ZERO, ZERO2};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use paste::paste;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Plane2(pub V2, pub V2, pub V2, pub V2);

pub const XY2: Plane2 = Plane2(ZERO2, X2, Y2, V2(1, 1));

impl Plane2 {
    pub fn new(a: V2, b: V2) -> Self {
        Self(ZERO2, a, b, a + b)
    }

    pub fn map<F: FnMut(V2) -> V2>(mut self, mut f: F) -> Self {
        self.0 = f(self.0);
        self.1 = f(self.1);
        self.2 = f(self.2);
        self.3 = f(self.3);
        self
    }
}

impl From<Plane2> for [[i16; 2]; 4] {
    fn from(plane: Plane2) -> [[i16; 2]; 4] {
        [
            plane.0.into(),
            plane.1.into(),
            plane.2.into(),
            plane.3.into(),
        ]
    }
}

impl From<Plane3> for [[i16; 3]; 4] {
    fn from(plane: Plane3) -> [[i16; 3]; 4] {
        [
            plane.0.into(),
            plane.1.into(),
            plane.2.into(),
            plane.3.into(),
        ]
    }
}

macro_rules! impl_iter {
    ($plane:ident, $vec:ident) => {
        paste! {
            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub struct [<$plane Iter>] {
                current: usize,
                plane: $plane,
            }

            impl IntoIterator for $plane {
                type Item = $vec;
                type IntoIter = [<$plane Iter>];
                fn into_iter(self) -> Self::IntoIter {
                    Self::IntoIter {
                        current: 0,
                        plane: self,
                    }
                }
            }

            impl Iterator for [<$plane Iter>]{
                type Item = $vec;
                fn next(&mut self) -> Option<Self::Item> {
                    let res = match self.current {
                        0 => Some(self.plane.0),
                        1 => Some(self.plane.1),
                        2 => Some(self.plane.2),
                        3 => Some(self.plane.3),
                        _ => None,
                    };
                    if res.is_some() {
                        self.current += 1;
                    }
                    res
                }
            }
        }
    };
}

impl_iter!(Plane2, V2);
impl_iter!(Plane3, V3);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Plane3(pub V3, pub V3, pub V3, pub V3);

pub const XY: Plane3 = Plane3(ZERO, X, Y, V3(1, 1, 0));
pub const XZ: Plane3 = Plane3(ZERO, X, Z, V3(1, 0, 1));
pub const YZ: Plane3 = Plane3(ZERO, Y, Z, V3(0, 1, 1));

impl Plane3 {
    pub fn new(a: V3, b: V3) -> Self {
        Self(ZERO, a, b, a + b)
    }

    pub fn map<F: FnMut(V3) -> V3>(mut self, mut f: F) -> Self {
        self.0 = f(self.0);
        self.1 = f(self.1);
        self.2 = f(self.2);
        self.3 = f(self.3);
        self
    }

    pub fn project<F: FnMut(V3) -> V2>(self, mut f: F) -> Plane2 {
        let mut res = XY2;
        res.0 = f(self.0);
        res.1 = f(self.1);
        res.2 = f(self.2);
        res.3 = f(self.3);
        res
    }
}

macro_rules! impl_op {
    ($plane:ident, $vec:ident, $fn:ident, $op:tt) => {
        paste! {
            impl [<$fn:camel>]<i16> for $plane {
                type Output = Self;
                fn $fn(mut self, rhs: i16) -> Self {
                    self $op rhs;
                    self
                }
            }
            impl [<$fn:camel>]<$vec> for $plane {
                type Output = Self;
                fn $fn(mut self, rhs: $vec) -> Self {
                    self $op rhs;
                    self
                }
            }
            impl [<$fn:camel Assign>]<i16> for $plane {
                fn [<$fn _assign>](&mut self, rhs: i16) {
                    self.0 $op rhs;
                    self.1 $op rhs;
                    self.2 $op rhs;
                    self.3 $op rhs;
                }
            }
            impl [<$fn:camel Assign>]<$vec> for $plane {
                fn [<$fn _assign>](&mut self, rhs: $vec) {
                    self.0 $op rhs;
                    self.1 $op rhs;
                    self.2 $op rhs;
                    self.3 $op rhs;
                }
            }
        }
    };
}

impl_op!(Plane2, V2, add, +=);
impl_op!(Plane2, V2, sub, -=);
impl_op!(Plane2, V2, mul, *=);
impl_op!(Plane2, V2, div, /=);
impl_op!(Plane2, V2, rem, %=);
impl_op!(Plane3, V3, add, +=);
impl_op!(Plane3, V3, sub, -=);
impl_op!(Plane3, V3, mul, *=);
impl_op!(Plane3, V3, div, /=);
impl_op!(Plane3, V3, rem, %=);
