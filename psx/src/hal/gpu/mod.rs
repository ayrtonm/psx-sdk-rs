use crate::gpu::Command;

mod gp0;
mod gp1;
mod stat;

const fn command(cmd: Command, other_bits: Option<u32>) -> u32 {
    let other_bits = match other_bits {
        Some(bits) => bits,
        None => 0,
    };
    (cmd as u32) << 24 | other_bits
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn gpustat() {
        use crate::bios;
        use crate::hal::{Register, GPUSTAT};
        assert!(bios::gpu_get_status() == GPUSTAT::load().bits());
    }
}
