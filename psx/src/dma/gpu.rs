use super::{BaseAddress, BlockControl, BlockSize, ChannelControl, Direction, Step, SyncMode,
            Transfer};
use crate::gpu::primitive::OT;
use crate::gpu::{Clut, TexPage};
use crate::mmio::{dma, gpu};
use crate::tim::TIM;

impl dma::gpu::Channel {
    pub fn prepare_ot(&mut self, gp1: &mut gpu::GP1) -> &mut Self {
        gp1.dma_direction(2);
        self.block_control.set(BlockSize::LinkedList);
        self.channel_control
            .set_direction(Direction::FromMemory)
            .set_sync_mode(SyncMode::LinkedList);
        self
    }

    pub fn send<'a, const N: usize>(
        &mut self, ot: &'a OT<N>,
    ) -> Transfer<dma::gpu::ChannelControl, &'a OT<N>> {
        self.send_offset(ot, ot.len())
    }

    pub fn send_offset<'a, const N: usize>(
        &mut self, ot: &'a OT<N>, n: usize,
    ) -> Transfer<dma::gpu::ChannelControl, &'a OT<N>> {
        self.base_address.set(ot.entry(n));
        self.channel_control.start(ot)
    }

    pub fn load_tim(&mut self, tim: &TIM) -> (TexPage, Option<Clut>) {
        self.channel_control
            .set_direction(Direction::FromMemory)
            .set_step(Step::Forward)
            .set_chop(None)
            .set_sync_mode(SyncMode::Immediate);

        let texpage = tim.texpage();
        let clut = tim.clut();

        let bmp = tim.bitmap().data();
        self.base_address.set(bmp.as_ptr());
        self.block_control.set(bmp.len());
        self.channel_control.start(()).wait();

        tim.clut_bitmap().map(|clut| {
            self.base_address.set(clut.data().as_ptr());
            self.block_control.set(clut.data().len());
            self.channel_control.start(()).wait();
        });

        (texpage, clut)
    }
}
