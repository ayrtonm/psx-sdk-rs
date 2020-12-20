use super::{BaseAddress, BlockControl, Chop, Direction, Step, SyncMode};
use crate::graphics::OT;
use crate::mmio::dma;

impl_mut_value!(dma::otc::ChannelControl);
impl_dma_channel_control!(dma::otc::ChannelControl);

impl dma::otc::Channel {
    pub fn clear<const N: usize>(&mut self, ot: &OT<N>) -> Transfer<()> {
        self.base_address.set(ot.first_entry());
        self.block_control.set(N as u32);
        self.channel_control
            .get_mut()
            .sync_mode(SyncMode::Immediate)
            .step(Step::Backward)
            .start(())
    }
}
