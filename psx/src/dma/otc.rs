use super::{BaseAddress, BlockControl, ChannelControl};

use crate::mmio::Address;
use crate::value::LoadMut;

/// [OTC DMA base address](http://problemkaputt.de/psx-spx.htm#dmachannels) register at `0x1F80_10A0`.
/// Used to set the DMA channel's base address.
pub struct MADR;

/// [OTC DMA block control](http://problemkaputt.de/psx-spx.htm#dmachannels) register at `0x1F80_10A4`.
/// Used to set the DMA channel's block size.
pub struct BCR;

/// [OTC DMA channel control](http://problemkaputt.de/psx-spx.htm#dmachannels) register at `0x1F80_10A8`.
/// Used to control the DMA channel.
pub struct CHCR;

impl Address<u32> for MADR {
    const ADDRESS: u32 = 0x1F80_10E0;
}
impl LoadMut<u32> for MADR {}
impl BaseAddress for MADR {}

impl Address<u32> for BCR {
    const ADDRESS: u32 = 0x1F80_10E4;
}
impl LoadMut<u32> for BCR {}
impl BlockControl for BCR {}

impl Address<u32> for CHCR {
    const ADDRESS: u32 = 0x1F80_10E8;
}
impl LoadMut<u32> for CHCR {}
impl ChannelControl for CHCR {}
