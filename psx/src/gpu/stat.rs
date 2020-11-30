use crate::mmio::gpu;
use crate::mmio::register::Read;

impl gpu::Stat {
    pub fn sync(&self) {
        unsafe { while self.read() & (1 << 28) == 0 {} }
    }
}
