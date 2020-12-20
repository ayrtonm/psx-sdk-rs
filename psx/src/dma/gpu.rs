use super::{BaseAddress, BlockControl, BlockSize, ChannelControl, Direction, Step, SyncMode};
use crate::gpu::{Clut, TexPage};
use crate::graphics::OT;
use crate::mmio::{dma, gpu};
use crate::tim::TIM;

impl_mut_value!(dma::gpu::ChannelControl);

// Let's make these methods more readable
type Transfer<'a, T> = super::Transfer<'a, dma::gpu::ChannelControl, T>;
type MaybeTransfer<'a, T> = super::MaybeTransfer<'a, dma::gpu::ChannelControl, T>;

impl dma::gpu::Channel {
    pub fn prepare_ot(&mut self, gp1: &mut gpu::GP1) -> &mut Self {
        gp1.dma_direction(2);
        self.block_control.set(BlockSize::LinkedList);
        self.channel_control
            .set_direction(Direction::FromMemory)
            .set_sync_mode(SyncMode::LinkedList);
        self
    }

    pub fn send<'a, const N: usize>(&mut self, ot: &'a OT<N>) -> Transfer<&'a OT<N>> {
        self.send_offset(ot, ot.start())
    }

    pub fn send_offset<'a, const N: usize>(
        &mut self, ot: &'a OT<N>, n: usize,
    ) -> Transfer<&'a OT<N>> {
        self.base_address.set(ot.entry(n));
        self.channel_control.start(ot)
    }

    pub fn load_tim<'a>(
        &mut self, tim: &TIM,
    ) -> Transfer<fn(&'a mut Self, &'a TIM) -> MaybeTransfer<'a, (TexPage, Option<Clut>)>> {
        self.channel_control
            .set_direction(Direction::FromMemory)
            .set_step(Step::Forward)
            .set_chop(None)
            .set_sync_mode(SyncMode::Immediate);

        let bmp = tim.bitmap().data();
        self.base_address.set(bmp.as_ptr());
        self.block_control.set(bmp.len());

        self.channel_control.start(|gpu_dma, tim| {
            let texpage = tim.texpage();
            let clut = tim.clut();
            let result = (texpage, clut);
            tim.clut_bitmap()
                .map(move |clut| {
                    gpu_dma.base_address.set(clut.data().as_ptr());
                    gpu_dma.block_control.set(clut.data().len());
                    MaybeTransfer::Transfer(gpu_dma.channel_control.start(result))
                })
                .unwrap_or(MaybeTransfer::Result(result))
        })
    }
}
