use super::{Direction, Step, TransferMode, GPU};
use crate::gpu::DMAMode;
use crate::hal::dma::{ChannelControl, MemoryAddress, SharedChannelControl};
use crate::hal::{MutRegister, GP1};

impl GPU {
    // This works even if `list` is stack allocated because it's blocking.
    pub fn send_list<L>(&mut self, list: *const L) {
        GP1.dma_mode(Some(DMAMode::GP0));
        // This should be replaced by two compiler fences, but it turns out that
        // LLVM emits `sync` instead of a pseudo-instruction for compiler fences
        // so I can't use it for MIPS-I. See the Embedonomicon's DMA chapter for
        // details.
        let list = unsafe { core::ptr::read_volatile(list) };
        let ptr = &list as *const L as *const u32;
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
