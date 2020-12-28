use super::{Direction, PackedVertex};

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
        unsafe { self.write(0x0000_0000) };
        self
    }

    /// Resets the GPU command buffer with GP1 command 0x01.
    #[inline(always)]
    pub fn reset_command_buffer(&mut self) -> &mut Self {
        unsafe { self.write(0x0100_0000) };
        self
    }

    /// Sets the display mask with GP1 command 0x03.
    #[inline(always)]
    pub fn display_enable(&mut self, enable: bool) -> &mut Self {
        unsafe { self.write(0x0300_0000 | !enable as u32) };
        self
    }

    /// Sets the DMA direction with GP1 command 0x04.
    #[inline(always)]
    pub fn dma_direction(&mut self, dir: Direction) -> &mut Self {
        unsafe { self.write(0x0400_0000 | dir as u32) }
        self
    }

    /// Sets the start of the display area with GP1 command 0x05.
    #[inline(always)]
    pub fn start_display_area(&mut self, start: PackedVertex<10, 9>) -> &mut Self {
        unsafe { self.write(0x0500_0000 | start.as_u32()) }
        self
    }

    /// Sets the horizontal display range with GP1 command 0x06.
    #[inline(always)]
    pub fn horizontal_range(&mut self, range: PackedVertex<12, 12>) -> &mut Self {
        unsafe { self.write(0x0600_0000 | range.as_u32()) }
        self
    }

    /// Sets the vertical display range with GP1 command 0x07.
    #[inline(always)]
    pub fn vertical_range(&mut self, range: PackedVertex<10, 10>) -> &mut Self {
        unsafe { self.write(0x0700_0000 | range.as_u32()) }
        self
    }
}
