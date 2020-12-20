use crate::mmio::gpu;
use crate::mmio::register::Read;

impl_value!(gpu::Stat);

// TODO: Remove this after making the gpu::Stat::Value methods from the consts
#[allow(dead_code)]
impl gpu::Stat {
    // TODO: Turn these into gpu::Stat::Value methods
    const TEX_PAGE_X: u32 = 0xF;
    const TEX_PAGE_Y: u32 = 1 << 4;
    const SEMI_TRANSPARENCY: u32 = 3 << 5;
    const TEX_PAGE_COLOR: u32 = 3 << 7;
    const DITHER: u32 = 1 << 9;
    const DRAWING_ENABLED: u32 = 1 << 10;
    const MASK_BIT: u32 = 1 << 11;
    const DRAW_MASKED: u32 = 1 << 12;
    const INTERLACE_FIELD: u32 = 1 << 13;
    //const REVERSE_FLAG: u32 = 1 << 14;
    const TEXTURE_DISABLE: u32 = 1 << 15;
    const HORIZONTAL_RES2: u32 = 1 << 16;
    const HORIZONTAL_RES1: u32 = 3 << 17;
    const VERTICAL_RES: u32 = 1 << 19;
    const VIDEO_MODE: u32 = 1 << 20;
    const DISPLAY_DEPTH: u32 = 1 << 21;
    const VERTICAL_INTERLACE: u32 = 1 << 22;
    const DISPLAY_ENABLED: u32 = 1 << 23;
    const IRQ: u32 = 1 << 24;
    const READY_FIFO: u32 = 1 << 25;
    const READY_CMD: u32 = 1 << 26;
    const READY_CPU: u32 = 1 << 27;
    const READY_DMA: u32 = 1 << 28;
    const DMA_DIRECTION: u32 = 3 << 29;
    const INTERLACE_PARITY: u32 = 1 << 31;

    // This method has to reread from memory which is why it's not implemented for
    // gpu::Stat::Value
    #[inline(always)]
    pub fn sync(&self) {
        while self.get().bits & (1 << 28) == 0 {}
    }
}
