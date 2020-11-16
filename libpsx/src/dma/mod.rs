use crate::rw_register;

pub mod gpu;

rw_register!(GpuDmaAddr, 0x1F80_10A0);
rw_register!(GpuDmaBlock, 0x1F80_10A4);
rw_register!(GpuDmaControl, 0x1F80_10A8);

pub enum DmaBlocks {
    // TODO: this should be u32 since 0x10000 is valid and gets mapped to 0u16
    Words(u16),
    Blocks { words: u16, blocks: u16 },
    LinkedList,
}
