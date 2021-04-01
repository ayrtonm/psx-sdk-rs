use super::{Channel, ChannelName, Direction};
use super::{MDECIn, MDECOut, Step, TransferMode, CDROM, GPU, OTC, PIO, SPU};
use crate::hal::dma::{BlockControl, ChannelControl, MemoryAddress};
use crate::hal::{MutRegister, Mutable, Register, Shared, DPCR};

pub trait Name {
    const NAME: ChannelName;
}

impl Name for MDECIn {
    const NAME: ChannelName = ChannelName::MDECIn;
}
impl Name for MDECOut {
    const NAME: ChannelName = ChannelName::MDECOut;
}
impl Name for GPU {
    const NAME: ChannelName = ChannelName::GPU;
}
impl Name for CDROM {
    const NAME: ChannelName = ChannelName::CDROM;
}
impl Name for SPU {
    const NAME: ChannelName = ChannelName::SPU;
}
impl Name for PIO {
    const NAME: ChannelName = ChannelName::PIO;
}
impl Name for OTC {
    const NAME: ChannelName = ChannelName::OTC;
}

impl<MADR, BCR, CHCR> Channel<MADR, BCR, CHCR>
where
    MADR: MemoryAddress,
    BCR: BlockControl,
    CHCR: ChannelControl,
    Self: Name,
{
    pub fn enabled() -> bool {
        DPCR::<Shared>::load().enabled(Self::NAME)
    }

    /// Enables the channel and returns its registers.
    pub fn channel() -> Self {
        DPCR::<Mutable>::load().enable(Self::NAME).store();
        Self::skip_enable()
    }

    pub fn reload(&mut self) {
        self.madr.reload();
        self.bcr.reload();
        self.chcr.reload();
    }

    /// Returns the channel's registers without enabling it.
    pub fn skip_enable() -> Self {
        Channel {
            madr: MADR::skip_load(),
            bcr: BCR::skip_load(),
            chcr: CHCR::load(),
        }
    }

    /// Sends `buffer` to the channel.
    pub fn send<'b>(&mut self, buffer: &'b [u32]) {
        self.chcr
            .set_direction(Direction::FromMemory)
            .set_step(Step::Forward)
            .set_mode(TransferMode::Immediate);
        self.madr.set(buffer.as_ptr() as u32).store();
        self.bcr.set_block(buffer.len()).store();
        self.chcr.start().store().wait();
    }
}
