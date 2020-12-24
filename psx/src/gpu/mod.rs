use crate::mmio::{Address, Write};

/// GPU Status register.
pub mod stat;

/// [GP0](http://problemkaputt.de/psx-spx.htm#gpuioportsdmachannelscommandsvram) register at `0x1F80_1810`.
/// Used to send commands for rendering and VRAM access.
pub struct GP0;

impl Address<u32> for GP0 {
    const ADDRESS: u32 = 0x1F80_1810;
}

impl Write<u32> for GP0 {}

/// [GP1](http://problemkaputt.de/psx-spx.htm#gpuioportsdmachannelscommandsvram) register at `0x1F80_1814`.
/// Used to send commands for display and DMA control.
pub struct GP1;

impl Address<u32> for GP1 {
    const ADDRESS: u32 = 0x1F80_1814;
}

impl Write<u32> for GP1 {}
