use ty::Command;

mod gp0;
mod gp1;
mod stat;
mod tests;
pub(crate) mod ty;

/// Gets a slice of the primitive in a struct
pub trait Primitive: Sized {
    fn primitive(&self) -> &[u32];
}

const fn command(cmd: Command, other_bits: Option<u32>) -> u32 {
    let other_bits = match other_bits {
        Some(bits) => bits,
        None => 0,
    };
    (cmd as u32) << 24 | other_bits
}
