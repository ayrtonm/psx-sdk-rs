use crate::gpu::GpuStat;
use crate::registers::{BitTwiddle, Read};

impl GpuStat {
    pub fn ready(&self) -> bool {
        self.read().bit(26) == 1
    }
}
