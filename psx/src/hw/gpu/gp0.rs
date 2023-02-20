use crate::gpu::{Color, Vertex};
use crate::hw::gpu::{GP0Command, GP0};
use crate::hw::Register;

impl GP0 {
    ///  Requests interrupt [`IRQ::GPU`][crate::hw::irq::IRQ::GPU].
    pub fn interrupt_request(&mut self) -> &mut Self {
        self.assign(0x1F << 24).store();
        self
    }

    /// Inserts a NOP which may be used for timing in the GPU buffer.
    pub fn nop(&mut self) -> &mut Self {
        self.assign(0x00 << 24).store();
        self
    }

    /// Fill a rectangle in VRAM with a [`Color`].
    pub fn fill_rectangle(&mut self, color: Color, offset: Vertex, size: Vertex) -> &mut Self {
        self.assign(0x02 << 24 | u32::from(color))
            .store()
            .assign(u32::from(offset))
            .store()
            .assign(u32::from(size))
            .store();
        self
    }

    /// Sends the GP0 command `cmd` to the GPU.
    ///
    /// # Safety
    ///
    /// Make sure that the GPU buffer has room for the command to avoid
    /// overflow. `GP0Command` types larger than the GPU buffer may not be sent
    /// without dropping some commands. This will not corrupt memory, but may
    /// cause unintended on-screen effects.
    // TODO: Make this unsafe
    pub fn send_command<C: GP0Command + ?Sized>(&mut self, cmd: &C) -> &mut Self {
        for &word in cmd.data() {
            self.assign(word).store();
        }
        self
    }
}
