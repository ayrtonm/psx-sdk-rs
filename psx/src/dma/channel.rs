use super::{Channel, Direction, Name};
use super::{Step, TransferMode};
use crate::hal::dma::{BlockControl, ChannelControl, MemoryAddress};
use crate::hal::{MutRegister, Mutable, Register, Shared, DPCR};

impl<MADR, BCR, CHCR, const NAME: Name> Channel<MADR, BCR, CHCR, NAME>
where
    MADR: MemoryAddress,
    BCR: BlockControl,
    CHCR: ChannelControl,
{
    pub fn enabled() -> bool {
        DPCR::<Shared>::load().enabled(NAME)
    }

    /// Enables the channel and returns its registers.
    pub fn new() -> Self {
        DPCR::<Mutable>::load().enable(NAME).store();
        Self::skip_enable()
    }

    pub fn reload(&mut self) {
        self.madr.reload();
        self.bcr.reload();
        self.chcr.reload();
    }

    /// Returns the channel's registers without enabling it.
    pub fn skip_enable() -> Self {
        // TODO: reconsider if this function should load MADR and BCR
        Channel {
            madr: unsafe { MADR::skip_load() },
            bcr: unsafe { BCR::skip_load() },
            chcr: CHCR::load(),
        }
    }

    pub fn split(self) -> (MADR, BCR, CHCR) {
        (self.madr, self.bcr, self.chcr)
    }

    /// Sends `buffer` to the channel.
    pub fn send<'b>(&mut self, buffer: &'b [u32]) {
        self.chcr
            .set_direction(Direction::FromMemory)
            .set_step(Step::Forward)
            .set_mode(TransferMode::Immediate);
        self.madr.set_bits(buffer.as_ptr() as u32).store();
        self.bcr.set_block(buffer.len()).store();
        self.chcr.start().store().wait();
    }
}
