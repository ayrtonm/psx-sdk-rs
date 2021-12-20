use crate::gpu::vertex::Error;
use crate::gpu::DispEnv;
use crate::gpu::{DMAMode, Depth, PackedVertex, Vertex, VideoMode};
use crate::hw::gpu::GP1;
use crate::hw::{MemRegister, Register};
use core::convert::TryFrom;

type Result<T> = core::result::Result<T, Error>;

impl GP1 {
    pub fn new() -> Self {
        GP1(MemRegister::skip_load())
    }

    pub fn reset_gpu(&mut self) -> &mut Self {
        self.0.assign(0x00 << 24).store();
        self
    }

    pub fn reset_buffer(&mut self) -> &mut Self {
        self.0.assign(0x01 << 24).store();
        self
    }

    pub fn ack_irq(&mut self) -> &mut Self {
        self.0.assign(0x02 << 24).store();
        self
    }

    pub fn enable_display(&mut self, enabled: bool) -> &mut Self {
        self.0.assign((0x03 << 24) | !enabled as u32).store();
        self
    }

    pub fn dma_mode(&mut self, direction: Option<DMAMode>) -> &mut Self {
        let mode = direction.map(|d| d as u32).unwrap_or(0);
        self.0.assign((0x04 << 24) | mode).store();
        self
    }

    /// The `start` tuple has fields restricted to (9 bits, 10 bits).
    pub fn display_start(&mut self, start: (i16, i16)) -> Result<&mut Self> {
        let start = PackedVertex::<3, 10, 9>::try_from(start)?;
        self._display_start(start)
    }

    fn _display_start(&mut self, start: PackedVertex<3, 10, 9>) -> Result<&mut Self> {
        self.0.assign((0x05 << 24) | u32::from(start)).store();
        Ok(self)
    }

    /// The `range` tuple has fields restricted to (12 bits, 12 bits).
    pub fn horizontal_range(&mut self, range: (i16, i16)) -> Result<&mut Self> {
        let range = PackedVertex::<3, 12, 12>::try_from(range)?;
        self._horizontal_range(range)
    }

    fn _horizontal_range(&mut self, range: PackedVertex<3, 12, 12>) -> Result<&mut Self> {
        self.0.assign((0x06 << 24) | u32::from(range)).store();
        Ok(self)
    }

    /// The `range` tuple has fields restricted to (10 bits, 10 bits).
    pub fn vertical_range(&mut self, range: (i16, i16)) -> Result<&mut Self> {
        let range = PackedVertex::<3, 10, 10>::try_from(range)?;
        self._vertical_range(range)
    }

    fn _vertical_range(&mut self, range: PackedVertex<3, 10, 10>) -> Result<&mut Self> {
        self.0.assign((0x07 << 24) | u32::from(range)).store();
        Ok(self)
    }

    /// The x resolution is restricted to 256, 320, 512, 640 or 368. The y
    /// resolution is restricted to 240 or 480.
    pub fn display_mode(
        &mut self, res: (i16, i16), mode: VideoMode, depth: Depth, interlace: bool,
    ) -> Result<&mut Self> {
        let res = Vertex::from(res);
        let hres = match res.x {
            256 => 0,
            320 => 1,
            512 => 2,
            640 => 3,
            368 => 1 << 6,
            _ => return Err(Error::InvalidX),
        };
        let vres = match res.y {
            240 => 0,
            480 => 1,
            _ => return Err(Error::InvalidY),
        };
        let settings =
            hres | vres << 2 | (mode as u32) << 3 | (depth as u32) << 4 | (interlace as u32) << 5;
        self.0.assign((0x08 << 24) | settings).store();
        Ok(self)
    }

    pub fn set_display_env(&mut self, disp_env: &DispEnv) -> &mut Self {
        self._display_start(disp_env.offset)
            .expect("")
            ._horizontal_range(disp_env.horizontal_range)
            .expect("")
            ._vertical_range(disp_env.vertical_range)
            .expect("")
    }
}
