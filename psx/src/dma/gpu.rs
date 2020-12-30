use super::{BaseAddress, BlockControl, BlockMode, ChannelControl, Direction, Transfer,
            TransferMode};

use crate::gpu;
use crate::gpu::GP1;
use crate::graphics::LinkedList;
use crate::mmio::Address;
use crate::value::LoadMut;

/// [GPU DMA base address](http://problemkaputt.de/psx-spx.htm#dmachannels) register at `0x1F80_10A0`.
/// Used to set the DMA channel's base address.
pub struct MADR;

/// [GPU DMA block control](http://problemkaputt.de/psx-spx.htm#dmachannels) register at `0x1F80_10A4`.
/// Used to set the DMA channel's block size.
pub struct BCR;

/// [GPU DMA channel control](http://problemkaputt.de/psx-spx.htm#dmachannels) register at `0x1F80_10A8`.
/// Used to control the DMA channel.
pub struct CHCR(());

impl CHCR {
    /// Creates a new instance of the GPU DMA channel's control register. Take
    /// care to only call this once.
    #[inline(always)]
    pub const unsafe fn new() -> Self {
        CHCR(())
    }

    /// Sends a linked list to the GPU.
    pub fn send_list<'l, L: LinkedList>(&mut self, list: &'l L) -> Transfer<&'l L, Self> {
        GP1.dma_direction(gpu::Direction::ToGPU);
        //while !GPUSTAT.load().cmd_ready() {}
        MADR.set(list.start_address());
        BCR.set(BlockMode::LinkedList);
        self.load_mut()
            .direction(Direction::FromMemory)
            .transfer_mode(TransferMode::LinkedList)
            .start(list)
    }
}

impl Address<u32> for MADR {
    const ADDRESS: u32 = 0x1F80_10A0;
}
impl LoadMut<u32> for MADR {}
impl BaseAddress for MADR {}

impl Address<u32> for BCR {
    const ADDRESS: u32 = 0x1F80_10A4;
}
impl LoadMut<u32> for BCR {}
impl BlockControl for BCR {}

impl Address<u32> for CHCR {
    const ADDRESS: u32 = 0x1F80_10A8;
}
impl LoadMut<u32> for CHCR {}
impl ChannelControl for CHCR {}
