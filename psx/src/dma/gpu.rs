use super::{BaseAddress, BlockControl, BlockSize, Chop, Direction, Step, SyncMode};
use crate::gpu::{Clut, TexPage};
use crate::graphics::OT;
use crate::mmio::{dma, gpu};
use crate::tim::TIM;

impl_mut_value!(dma::gpu::ChannelControl);
impl_dma_channel_control!(dma::gpu::ChannelControl);

impl dma::gpu::Channel {
    #[inline(always)]
    pub fn prepare_ot(&mut self, gp1: &mut gpu::GP1) -> &mut Self {
        gp1.dma_direction(2);
        self.block_control.set(BlockSize::LinkedList);
        self.channel_control
            .get_mut()
            .direction(Direction::FromMemory)
            .sync_mode(SyncMode::LinkedList)
            .set();
        self
    }

    pub fn send<'a, const N: usize>(&mut self, ot: &'a OT<N>) -> Transfer<&'a OT<N>> {
        self.send_offset(ot, ot.start())
    }

    pub fn send_offset<'a, const N: usize>(
        &mut self, ot: &'a OT<N>, n: usize,
    ) -> Transfer<&'a OT<N>> {
        self.base_address.set(ot.entry(n));
        self.channel_control.get_mut().start(ot)
    }

    pub fn load_tim<'a>(
        &mut self, tim: &TIM,
    ) -> Transfer<fn(&'a mut Self, &'a TIM) -> MaybeTransfer<'a, (TexPage, Option<Clut>)>> {
        let mut_val = self
            .channel_control
            .get_mut()
            .direction(Direction::FromMemory)
            .step(Step::Forward)
            .chop(None)
            .sync_mode(SyncMode::Immediate);

        let bmp = tim.bitmap().data();
        self.base_address.set(bmp.as_ptr());
        self.block_control.set(bmp.len());

        mut_val.start(|gpu_dma, tim| {
            let texpage = tim.texpage();
            let clut = tim.clut();
            let result = (texpage, clut);
            tim.clut_bitmap()
                .map(move |clut| {
                    gpu_dma.base_address.set(clut.data().as_ptr());
                    gpu_dma.block_control.set(clut.data().len());
                    MaybeTransfer::Transfer(gpu_dma.channel_control.get_mut().start(result))
                })
                .unwrap_or(MaybeTransfer::Result(result))
        })
    }
}
