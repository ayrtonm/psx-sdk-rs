use bitflags::bitflags;

use crate::mmio::gpu;
use crate::mmio::register::Read;

bitflags! {
    // TODO: Double check the meaning of each flag
    pub struct StatusFlags: u32 {
        const TEX_PAGE_X = 0xF;
        const TEX_PAGE_Y = 1 << 4;
        const SEMI_TRANSPARENCY = 3 << 5;
        const TEX_PAGE_COLOR = 3 << 7;
        const DITHER = 1 << 9;
        const DRAWING_ENABLED = 1 << 10;
        const MASK_BIT = 1 << 11;
        const DRAW_MASKED = 1 << 12;
        const INTERLACE_FIELD = 1 << 13;
        //consT REVERSE_FLAG = 1 << 14;
        const TEXTURE_DISABLE = 1 << 15;
        const HORIZONTAL_RES2 = 1 << 16;
        const HORIZONTAL_RES1 = 3 << 17;
        const VERTICAL_RES = 1 << 19;
        const VIDEO_MODE = 1 << 20;
        const DISPLAY_DEPTH = 1 << 21;
        const VERTICAL_INTERLACE = 1 << 22;
        const DISPLAY_ENABLED = 1 << 23;
        const IRQ = 1 << 24;
        const READY_FIFO = 1 << 25;
        const READY_CMD = 1 << 26;
        const READY_CPU = 1 << 27;
        const READY_DMA = 1 << 28;
        const DMA_DIRECTION = 3 << 29;
        const INTERLACE_PARITY = 1 << 31;
    }
}

impl gpu::Stat {
    // Only used allow "shadowing" the internal Read::read with the bitflags
    // version. This isn't really shadowing since the interal `read` can't be
    // called directly or implemented.
    unsafe fn read_value(&self) -> u32 {
        <Self as Read<u32>>::read(self)
    }

    pub fn read(&self) -> StatusFlags {
        unsafe { StatusFlags::from_bits_unchecked(self.read_value()) }
    }

    pub fn sync(&self) {
        //while !self.read().contains(StatusFlags::READY_DMA) {}
        //unsafe { while self.read_value() & (1 << 28) == 0 {} }
    }
}
