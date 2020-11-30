use super::{BaseAddress, BlockControl, ChannelControl, Step, SyncMode, Transfer};
use crate::gpu::primitive::OT;
use crate::mmio::dma;

impl dma::otc::Channel {
    pub fn clear<const N: usize>(&mut self, ot: &OT<N>) -> Transfer<dma::otc::ChannelControl, ()> {
        self.base_address.set(ot.start());
        self.block_control.set(N as u32);
        self.channel_control
            .set_sync_mode(SyncMode::Immediate)
            .set_step(Step::Backward)
            .start(())
    }
}
