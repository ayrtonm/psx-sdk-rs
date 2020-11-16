pub mod gpu;

pub use super::registers::dma::*;

pub enum DmaBlocks {
    // TODO: this should be u32 since 0x10000 is valid and gets mapped to 0u16
    Words(u16),
    Blocks { words: u16, blocks: u16 },
    LinkedList,
}
