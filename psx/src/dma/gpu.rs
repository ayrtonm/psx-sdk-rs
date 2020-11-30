use super::{BaseAddress, BlockControl, BlockSize, ChannelControl, Direction, Step, SyncMode,
            Transfer};
use crate::gpu::primitive::OT;
use crate::gpu::texture::{Clut, TexPage};
use crate::mmio::register::Write;
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

    pub fn send<const N: usize>(&mut self, ot: &OT<N>) -> Transfer<dma::gpu::ChannelControl, ()> {
        self.send_offset(ot, N - 1)
    }

    pub fn send_offset<const N: usize>(
        &mut self, ot: &OT<N>, n: usize,
    ) -> Transfer<dma::gpu::ChannelControl, ()> {
        self.base_address.set(ot.entry(n));
        self.channel_control.start(())
    }

    // TODO: passing GP0 isn't necessary if the transfers are purely done thru DMA
    // This should be easy to do by increasing the size of the TIM array to include a 1 or 2 word
    // header. This'll give flexibility for prepending 0xA000_0000 or prepending a linked list tag +
    // 0xA000_0000.
    pub fn load_tim(&mut self, tim: &TIM, gp0: &mut gpu::GP0) -> (TexPage, Option<Clut>) {
        self.channel_control
            .set_direction(Direction::FromMemory)
            .set_step(Step::Forward)
            .set_chop(None)
            .set_sync_mode(SyncMode::Immediate);

        let texpage = tim.texpage();
        let clut = tim.clut();

        unsafe { gp0.write(0xA0 << 24) };
        let bmp = tim.bitmap().data();
        self.base_address.set(bmp.as_ptr());
        self.block_control.set(bmp.len());
        self.channel_control.start(()).wait();

        tim.clut_bitmap().map(|clut| {
            unsafe { gp0.write(0xA0 << 24) };
            self.base_address.set(clut.data().as_ptr());
            self.block_control.set(clut.data().len());
            self.channel_control.start(()).wait();
        });

        (texpage, clut)
    }
}
