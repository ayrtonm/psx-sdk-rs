use super::{Direction, TransferMode, GPU};
use crate::gpu::DMAMode;
use crate::hal::dma::{ChannelControl, MemoryAddress, SharedChannelControl};
use crate::hal::{MutRegister, GP1};

impl GPU {
    pub fn send_list<L>(&mut self, list: &L) {
        GP1.dma_mode(Some(DMAMode::GP0));
        self.madr
            .set_address(list as *const L as *const u32)
            .store();
        self.chcr
            .set_direction(Direction::FromMemory)
            .set_mode(TransferMode::LinkedList)
            .start()
            .store()
            .wait();
    }
}
