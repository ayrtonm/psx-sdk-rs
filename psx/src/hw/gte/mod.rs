//! Geometry Transformation Engine Coprocessor
//!
//! This module provides access to GTE, or cop2, registers and instructions.

use crate::hw::Register;

define_cop! {
    VXY0<u32>; COP: 2; R: 0,
    VZ0<i16>;  COP: 2; R: 1,
    VXY1<u32>; COP: 2; R: 2,
    VZ1<i16>;  COP: 2; R: 3,
    VXY2<u32>; COP: 2; R: 4,
    VZ2<i16>;  COP: 2; R: 5,

    /// Ordering table average Z value
    OTZ<u16>;  COP: 2; R: 7,

    MAC0<i32>; COP: 2; R: 24,

    MAC1<i32>; COP: 2; R: 25,
    MAC2<i32>; COP: 2; R: 26,
    MAC3<i32>; COP: 2; R: 27,

    /// Leading zeros count source
    LZCS<u32>; COP: 2; R: 30,
    /// Leading zeros count result
    LZCR<u32>; COP: 2; R: 31,

    /// Rotation matrix entries RT11 and RT12
    RT11_12<u32>; COP: 2; R: 32,
    /// Rotation matrix entries RT13 and RT21
    RT13_21<u32>; COP: 2; R: 33,
    /// Rotation matrix entries RT22 and RT23
    RT22_23<u32>; COP: 2; R: 34,
    /// Rotation matrix entries RT31 and RT32
    RT31_32<u32>; COP: 2; R: 35,
    /// Rotation matrix entry RT33
    RT33<i16>;    COP: 2; R: 36,

    /// Light matrix entries L11 and L12
    L11_12<u32>; COP: 2; R: 40,
    /// Light matrix entries L13 and L21
    L13_21<u32>; COP: 2; R: 41,
    /// Light matrix entries L22 and L23
    L22_23<u32>; COP: 2; R: 42,
    /// Light matrix entries L31 and L32
    L31_32<u32>; COP: 2; R: 43,
    /// Light matrix entry L33
    L33<i16>;    COP: 2; R: 44,

    /// Light color matrix entries LR11 and LR12
    LR11_12<u32>; COP: 2; R: 48,
    /// Light color matrix entries LR13 and LR21
    LR13_21<u32>; COP: 2; R: 49,
    /// Light color matrix entries LR22 and LR23
    LR22_23<u32>; COP: 2; R: 50,
    /// Light color matrix entries LR31 and LR32
    LR31_32<u32>; COP: 2; R: 51,
    /// Light color matrix entry LR33
    LR33<i16>; COP: 2; R: 52,
}

pub trait VectorXY: Register<u32> {}
pub trait VectorZ: Register<i16> {}

pub trait MatrixAB: Register<u32> {}
pub trait MatrixC: Register<i16> {}

impl VectorXY for VXY0 {}
impl VectorXY for VXY1 {}
impl VectorXY for VXY2 {}

impl VectorZ for VZ0 {}
impl VectorZ for VZ1 {}
impl VectorZ for VZ2 {}

impl MatrixAB for RT11_12 {}
impl MatrixAB for RT13_21 {}
impl MatrixAB for RT22_23 {}
impl MatrixAB for RT31_32 {}
impl MatrixC for RT33 {}

impl MatrixAB for L11_12 {}
impl MatrixAB for L13_21 {}
impl MatrixAB for L22_23 {}
impl MatrixAB for L31_32 {}
impl MatrixC for L33 {}

impl MatrixAB for LR11_12 {}
impl MatrixAB for LR13_21 {}
impl MatrixAB for LR22_23 {}
impl MatrixAB for LR31_32 {}
impl MatrixC for LR33 {}
