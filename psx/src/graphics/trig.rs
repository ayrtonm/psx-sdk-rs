use crate::graphics::f16;

pub fn cos(x: f16) -> f16 {
    f16::from(libm::cosf(f32::from(x)))
}

pub fn sin(x: f16) -> f16 {
    f16::from(libm::sinf(f32::from(x)))
}
