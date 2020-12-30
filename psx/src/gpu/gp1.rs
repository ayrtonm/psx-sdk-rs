use super::{cmd, Direction, PackedVertex};

use crate::mmio;
use crate::mmio::Address;
use crate::value::Write;

/// [GP1](http://problemkaputt.de/psx-spx.htm#gpuioportsdmachannelscommandsvram) register at `0x1F80_1814`.
/// Used to send commands for display and DMA control.
pub struct GP1;

impl Address<u32> for GP1 {
    const ADDRESS: u32 = 0x1F80_1814;
}

impl mmio::Write<u32> for GP1 {}

impl GP1 {
    /// Resets the GPU with GP1 command 0x00.
    #[inline(always)]
    pub fn reset_gpu(&mut self) -> &mut Self {
        unsafe { self.write(cmd(0)) };
        self
    }

    /// Resets the GPU command buffer with GP1 command 0x01.
    #[inline(always)]
    pub fn reset_command_buffer(&mut self) -> &mut Self {
        unsafe { self.write(cmd(0x01)) };
        self
    }

    /// Sets the display mask with GP1 command 0x03.
    #[inline(always)]
    pub fn display_enable(&mut self, enable: bool) -> &mut Self {
        unsafe { self.write(cmd(0x03) | !enable as u32) };
        self
    }

    /// Sets the DMA direction with GP1 command 0x04.
    #[inline(always)]
    pub fn dma_direction(&mut self, dir: Direction) -> &mut Self {
        unsafe { self.write(cmd(0x04) | dir as u32) }
        self
    }

    /// Sets the start of the display area with GP1 command 0x05.
    #[inline(always)]
    pub fn start_display_area(&mut self, start: PackedVertex<3, 10, 9>) -> &mut Self {
        unsafe { self.write(cmd(0x05) | start.as_u32()) }
        self
    }

    /// Sets the horizontal display range with GP1 command 0x06.
    #[inline(always)]
    pub fn horizontal_range(&mut self, range: PackedVertex<3, 12, 12>) -> &mut Self {
        unsafe { self.write(cmd(0x06) | range.as_u32()) }
        self
    }

    /// Sets the vertical display range with GP1 command 0x07.
    #[inline(always)]
    pub fn vertical_range(&mut self, range: PackedVertex<3, 10, 10>) -> &mut Self {
        unsafe { self.write(cmd(0x07) | range.as_u32()) }
        self
    }

    /// Sets the resolution, video mode, color depth and interlacing with GP1
    /// command 0x08.
    #[inline(always)]
    pub fn display_mode(&mut self) -> &mut Self {
        todo!("Implement this")
    }
}
