//! Geometry Transformation Engine Coprocessor
//!
//! This module provides access to GTE, or cop2, registers and instructions.

use crate::hw::Register;

define_cop! {
    /// The 16-bit VX0 and VY0 vectors
    VXY0<u32>; COP: 2; R: 0,
    /// The 16-bit VZ0 vector
    VZ0<i16>;  COP: 2; R: 1,
    /// The 16-bit VX1 and VY1 vectors
    VXY1<u32>; COP: 2; R: 2,
    /// The 16-bit VZ1 vector
    VZ1<i16>;  COP: 2; R: 3,
    /// The 16-bit VX2 and VY2 vectors
    VXY2<u32>; COP: 2; R: 4,
    /// The 16-bit VZ2 vector
    VZ2<i16>;  COP: 2; R: 5,

    /// Ordering table average Z value
    OTZ<u16>;  COP: 2; R: 7,

    /// Scalar math accumulator
    MAC0<i32>; COP: 2; R: 24,

    /// The first component of the vector math accumulator
    MAC1<i32>; COP: 2; R: 25,
    /// The second component of the vector math accumulator
    MAC2<i32>; COP: 2; R: 26,
    /// The third component of the vector math accumulator
    MAC3<i32>; COP: 2; R: 27,

    /// Leading zeros count source
    LZCS<u32>; COP: 2; R: 30,
    /// Leading zeros count result
    LZCR<u32>; COP: 2; R: 31,

    /*
    TODO: LLVM doesn't support these coprocessor instructions yet (#7).
    /// Rotation matrix entries RT11 and RT12
    RT11_12<u32>; COP: 2; R: 32; "c",
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
    */
}
