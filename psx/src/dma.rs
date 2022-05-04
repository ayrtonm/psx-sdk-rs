//! High-level DMA channel operations and types.

/// A DMA-specific error.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    /// The address is not 4-byte aligned.
    UnalignedAddress,
    /// The block size is too large.
    OversizedBlock,
}

/// Specifies the DMA channel's block mode, number and length.
#[derive(Debug)]
pub enum BlockMode {
    /// A single block of fixed size.
    Single(u32),
    /// Multiple blocks of fixed size.
    Multi {
        /// The size of each block.
        words: u16,
        /// The number of blocks.
        blocks: u16,
    },
    /// A variable number of variably-sized blocks represented by a linked-list.
    LinkedList,
}

impl From<u32> for BlockMode {
    fn from(words: u32) -> BlockMode {
        BlockMode::Single(words)
    }
}

impl From<usize> for BlockMode {
    fn from(words: usize) -> BlockMode {
        BlockMode::Single(words as u32)
    }
}

/// Specifies the DMA channel's transfer mode.
#[derive(Debug)]
pub enum TransferMode {
    /// Start transfer immediately and all at once.
    Immediate = 0,
    /// Sync blocks to DMA requests.
    Request,
    /// Transfer blocks in linked-list mode.
    LinkedList,
}

/// The DMA channel's transfer direction.
#[derive(Debug)]
pub enum Direction {
    /// To RAM from a device.
    ToMemory = 0,
    /// From RAM to a device.
    FromMemory,
}

/// The DMA channel's memory address step.
#[derive(Debug)]
pub enum Step {
    /// Step forwards by 4 bytes.
    Forward = 0,
    /// Step backwards by 4 bytes.
    Backward,
}

/// The DMA channel's CPU/transfer window sizes.
#[derive(Debug)]
pub struct Chop {
    /// The size of the DMA window.
    pub dma_window: u32,
    /// The size of the CPU window.
    pub cpu_window: u32,
}

/// A marker trait for DMA linked lists.
pub trait LinkedList {}
