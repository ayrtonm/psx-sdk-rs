use super::{Direction, Step, TransferMode, GPU};
use crate::gpu::{draw_sync, DMAMode};
use crate::hal::dma::{ChannelControl, MemoryAddress, SharedChannelControl};
use crate::hal::{MutRegister, GP1, GPUSTAT};

impl GPU {
    // This works even if `list` is stack allocated because it's blocking.
    pub fn send_list<L>(&mut self, list: &L) {
        GP1.dma_mode(Some(DMAMode::GP0));
        // TODO: is it necessary to reload `D2_CHCR` here?
        //self.reload();
        self.chcr.wait();
        let ptr = list as *const L as *const u32;
        self.madr.set_address(ptr).store();
        self.bcr.clear_all().store();
        self.chcr
            .set_step(Step::Forward)
            .set_chop(None)
            .set_direction(Direction::FromMemory)
            .set_mode(TransferMode::LinkedList)
            .start()
            .store()
            .wait();
    }
}
