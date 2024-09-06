use crate::gpu::{DMAMode, Depth, DispEnv, PackedVertex, VertexError, VideoMode};
use crate::hw::gpu::GP1;
use crate::hw::Register;
use core::result::Result;

impl GP1 {
    /// Resets the GPU.
    ///
    /// Clears the GPU buffer, acknowledges pending GPU interrupt request, turns
    /// the display off and disables DMA on the GPU side. Resets the display
    /// address to (0, 0) and the resolution to (320, 200) NTSC.
    pub fn reset_gpu(&mut self) -> &mut Self {
        self.assign(0x00 << 24).store();
        self
    }

    /// Empties the GPU buffer.
    pub fn reset_buffer(&mut self) -> &mut Self {
        self.assign(0x01 << 24).store();
        self
    }

    /// Acknowledges pending GPU interrupt request.
    pub fn ack_irq(&mut self) -> &mut Self {
        self.assign(0x02 << 24).store();
        self
    }

    /// Enables the display.
    pub fn enable_display(&mut self, enabled: bool) -> &mut Self {
        self.assign((0x03 << 24) | !enabled as u32).store();
        self
    }

    /// Set the GPU DMA direction.
    ///
    /// Setting `direction` to `None` disables DMA on the GPU side.
    pub fn dma_mode(&mut self, direction: Option<DMAMode>) -> &mut Self {
        let mode = direction.map(|d| d as u32).unwrap_or(0);
        self.assign((0x04 << 24) | mode).store();
        self
    }

    /// Sets the position of the display's top left corner.
    pub fn display_start(&mut self, start: PackedVertex<3, 10, 9>) -> &mut Self {
        self.assign((0x05 << 24) | u32::from(start)).store();
        self
    }

    /// Sets the horizontal display range.
    pub fn horizontal_range(&mut self, range: PackedVertex<3, 12, 12>) -> &mut Self {
        self.assign((0x06 << 24) | u32::from(range)).store();
        self
    }

    /// Sets the vertical display range.
    pub fn vertical_range(&mut self, range: PackedVertex<3, 10, 10>) -> &mut Self {
        self.assign((0x07 << 24) | u32::from(range)).store();
        self
    }

    /// The x resolution is restricted to 256, 320, 512, 640 or 368.
    /// The y resolution is restricted to 240 or 480 for NTSC,
    /// 256 or 512 for PAL.
    pub fn display_mode(
        &mut self, res: (i16, i16), mode: VideoMode, depth: Depth, interlace: bool,
    ) -> Result<&mut Self, VertexError> {
        let hres = match res.0 {
            256 => 0,
            320 => 1,
            512 => 2,
            640 => 3,
            368 => 1 << 6,
            _ => return Err(VertexError::InvalidX),
        };
        let vres = match (mode, res.1) {
            (VideoMode::NTSC, 240) | (VideoMode::PAL, 256) => 0,
            (VideoMode::NTSC, 480) | (VideoMode::PAL, 512) => 1,
            _ => return Err(VertexError::InvalidY),
        };
        let settings =
            hres | vres << 2 | (mode as u32) << 3 | (depth as u32) << 4 | (interlace as u32) << 5;
        self.assign((0x08 << 24) | settings).store();
        Ok(self)
    }

    /// Apply the settings in the display environment.
    pub fn set_display_env(&mut self, disp_env: &DispEnv) -> &mut Self {
        self.display_start(disp_env.offset)
            .horizontal_range(disp_env.horizontal_range)
            .vertical_range(disp_env.vertical_range)
    }
}
