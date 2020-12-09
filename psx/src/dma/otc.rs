use super::{BaseAddress, BlockControl, ChannelControl, Step, SyncMode, Transfer};
use crate::gpu::graphics::SingleOT;
use crate::mmio::{dma, Enabled};

impl dma::otc::Channel<Enabled> {
    pub fn clear<const N: usize>(
        &mut self, ot: &SingleOT<N>,
    ) -> Transfer<dma::otc::ChannelControl, ()> {
        self.base_address.set(ot.first_entry());
        self.block_control.set(N as u32);
        self.channel_control
            .set_sync_mode(SyncMode::Immediate)
            .set_step(Step::Backward)
            .start(())
    }
}
