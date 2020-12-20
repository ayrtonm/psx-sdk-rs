use super::{BaseAddress, BlockControl, BlockSize, Chop, Direction, Step, SyncMode};
use crate::graphics::OT;
use crate::mmio::dma;

impl_mut_value!(dma::otc::ChannelControl);
impl_dma_value!(dma::otc::ChannelControl);

impl dma::otc::Channel {
    pub fn clear<const N: usize>(&mut self, ot: &OT<N>) -> Transfer<()> {
        self.base_address.set(ot.first_entry());
        self.block_control.set(N as u32);
        self.channel_control
            .get_mut()
            .set_sync_mode(SyncMode::Immediate)
            .set_step(Step::Backward)
            .start(())
    }
}
