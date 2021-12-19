use crate::hw::gpu::Response;
use crate::hw::{MemRegister, Register};

impl Response {
    pub fn new() -> Self {
        Response(MemRegister::new())
    }

    pub fn load(&mut self) -> &mut Self {
        self.0.load();
        self
    }
}
