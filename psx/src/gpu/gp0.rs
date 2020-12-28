use crate::mmio;
use crate::mmio::Address;

/// [GP0](http://problemkaputt.de/psx-spx.htm#gpuioportsdmachannelscommandsvram) register at `0x1F80_1810`.
/// Used to send commands for rendering and VRAM access.
pub struct GP0;

impl Address<u32> for GP0 {
    const ADDRESS: u32 = 0x1F80_1810;
}

impl mmio::Write<u32> for GP0 {}
