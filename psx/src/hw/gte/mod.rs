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
}
