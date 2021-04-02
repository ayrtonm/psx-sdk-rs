#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Name {
    MDECIn = 0,
    MDECOut,
    GPU,
    CDROM,
    SPU,
    PIO,
    OTC,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlockMode {
    Single(u32),
    Multi { words: u16, blocks: u16 },
    LinkedList,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    ToMemory = 0,
    FromMemory,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Step {
    Forward = 0,
    Backward,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Chop {
    pub dma_win: u32,
    pub cpu_win: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TransferMode {
    Immediate = 0,
    Request,
    LinkedList,
}

impl From<usize> for BlockMode {
    fn from(words: usize) -> BlockMode {
        BlockMode::Single(words as u32)
    }
}

impl From<u32> for BlockMode {
    fn from(words: u32) -> BlockMode {
        BlockMode::Single(words)
    }
}
