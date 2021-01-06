use crate::gte::f16::f16;

/// Approximates sine
pub fn sin(mut x: f32) -> f32 {
    fn approx_sin(z: f32) -> f32 {
        4.0 * z * (180.0 - z) / (40500.0 - (z * (180.0 - z)))
    }
    while x < 0.0 {
        x += 360.0;
    }
    while x > 360.0 {
        x -= 360.0;
    }
    if x <= 180.0 {
        approx_sin(x)
    } else {
        -approx_sin(x - 180.0)
    }
}

/// Approximates cosine
pub fn cos(x: f32) -> f32 {
    let y = 90.0 - x;
    sin(y)
}

/// Approximates sine
pub fn fsin<F: Into<f16>>(x: F) -> f16 {
    let mut x = x.into();
    fn approx_sin(z: f16) -> f16 {
        let pi = 3.14159265359;
        let a = -1.5 + (pi / 2.0);
        let b = 2.5 - pi;
        let c = pi / 2.0;
        let z2 = z * z;
        let z3 = z2 * z;
        let z5 = z3 * z2;
        (c * z) + (b * z3) + (a * z5)
    }
    while x < f16(0.0) {
        x += 4.0;
    }
    while x > f16(4.0) {
        x -= 4.0;
    }
    // 0 to 1
    if f16(0.0) <= x && x <= f16(1.0) {
        approx_sin(x)
    // 1 to 2
    } else if x <= f16(2.0) {
        approx_sin(1.0 - (x - 1.0))
    // 2 to 3
    } else if x <= f16(3.0) {
        -approx_sin(x - 2.0)
    // 3 to 4
    } else if x <= f16(4.0) {
        -approx_sin(1.0 - (x - 3.0))
    } else {
        panic!("uh oh");
    }
}

/// Approximates cosine
pub fn fcos<F: Into<f16>>(x: F) -> f16 {
    let y = 1.0 - x.into();
    fsin(y)
}
