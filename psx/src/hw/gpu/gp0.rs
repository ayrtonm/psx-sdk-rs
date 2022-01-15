use crate::gpu::{Vertex,Color};
use crate::hw::gpu::{GP0Command, GP0};
use crate::hw::{MemRegister, Register};

impl GP0 {
    pub fn new() -> Self {
        GP0(MemRegister::skip_load())
    }

    pub fn interrupt_request(&mut self) -> &mut Self {
        self.0.assign(0x1F << 24).store();
        self
    }

    pub fn nop(&mut self) -> &mut Self {
        self.0.assign(0x00 << 24).store();
        self
    }

    pub fn fill_rectangle(&mut self, color: Color, offset: Vertex, size: Vertex) -> &mut Self {
        self.0
            .assign(0x02 << 24 | u32::from(color))
            .store()
            .assign(u32::from(offset))
            .store()
            .assign(u32::from(size))
            .store();
        self
    }

    pub fn send_command<C: GP0Command>(&mut self, cmd: &C) -> &mut Self {
        for &word in cmd.words() {
            self.0.assign(word).store();
        }
        self
    }
}
