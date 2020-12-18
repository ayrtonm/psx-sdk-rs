//! CPU-side DMA channel routines.

use crate::mmio::dma;
use crate::mmio::register::{Read, Update, Write};

mod gpu;
mod otc;

pub enum BlockSize {
    Single(usize),
    Multi { words: u16, blocks: u16 },
    LinkedList,
}

impl From<usize> for BlockSize {
    fn from(words: usize) -> Self {
        BlockSize::Single(words)
    }
}
impl From<u32> for BlockSize {
    fn from(words: u32) -> Self {
        BlockSize::Single(words as usize)
    }
}

pub enum Direction {
    ToMemory = 0,
    FromMemory,
}

pub enum Step {
    Forward = 0,
    Backward,
}

pub struct Chop {
    dma: u32,
    cpu: u32,
}

pub enum SyncMode {
    Immediate = 0,
    Request,
    LinkedList,
}

pub trait BaseAddress: Read<u32> + Write<u32> {
    /// Gets the memory address where this DMA channel will start reading
    /// from/writing to.
    fn get(&self) -> u32 {
        unsafe { self.read() }
    }

    /// Sets the memory address where this DMA channel will start reading
    /// from/writing to.
    fn set(&mut self, address: *const u32) {
        let address = address as u32;
        if cfg!(debug_assertions) {
            assert_eq!(address >> 24, 0);
        }
        unsafe { self.write(address) }
    }
}
pub trait BlockControl: Read<u32> + Write<u32> {
    fn get(&self, sync_mode: SyncMode) -> Option<BlockSize> {
        let value = unsafe { self.read() };
        match sync_mode {
            SyncMode::Immediate => match value {
                0 => Some(0x1_0000u32.into()),
                1..=0xFFFF => Some(value.into()),
                _ => None,
            },
            SyncMode::Request => Some(BlockSize::Multi {
                words: value as u16,
                blocks: (value >> 16) as u16,
            }),
            SyncMode::LinkedList => Some(BlockSize::LinkedList),
        }
    }
    fn set<T>(&mut self, block_size: T)
    where BlockSize: From<T> {
        let block_size = BlockSize::from(block_size);
        let words = match block_size {
            BlockSize::Single(words) => match words {
                0..=0xFFFF => words as u32,
                0x1_0000 => 0,
                _ => {
                    if cfg!(debug_assertions) {
                        panic!("Number of words can't exceed 0x1_0000");
                    };
                    0
                },
            },
            BlockSize::Multi { words, blocks } => words as u32 | ((blocks as u32) << 16),
            BlockSize::LinkedList => 0,
        };
        unsafe {
            self.write(words);
        }
    }
}
pub trait ChannelControl: Update<u32> {
    fn set_direction(&mut self, direction: Direction) -> &mut Self {
        unsafe {
            self.update(|val| val & !1 | (direction as u32));
        }
        self
    }
    fn set_step(&mut self, step: Step) -> &mut Self {
        unsafe {
            self.update(|val| val & !0b10 | ((step as u32) << 1));
        }
        self
    }
    fn set_chop(&mut self, chop: Option<Chop>) -> &mut Self {
        unsafe {
            self.update(|val| match chop {
                Some(chop) => {
                    if cfg!(debug_assertions) {
                        if chop.dma > 0b111 || chop.cpu > 0b111 {
                            panic!("DMA chopping windows are limited to 3 bits");
                        }
                    }
                    val | (1 << 8) | (chop.dma << 16) | (chop.cpu << 20)
                },
                None => val & !(1 << 8),
            })
        }
        self
    }
    fn set_sync_mode(&mut self, sync_mode: SyncMode) -> &mut Self {
        unsafe {
            self.update(|val| (val & !(0b11 << 9)) | ((sync_mode as u32) << 9));
        }
        self
    }
    fn sync_mode(&self) -> Option<SyncMode> {
        let bits = unsafe { self.read() };
        match (bits >> 9) & 0b11 {
            0 => Some(SyncMode::Immediate),
            1 => Some(SyncMode::Request),
            2 => Some(SyncMode::LinkedList),
            _ => None,
        }
    }
    fn start<T>(&mut self, result: T) -> Transfer<Self, T> {
        unsafe {
            match self.sync_mode() {
                Some(SyncMode::Immediate) => self.update(|val| val | (1 << 24) | (1 << 28)),
                _ => self.update(|val| val | (1 << 24)),
            }
        }
        Transfer {
            channel_control: self,
            result,
        }
    }
    fn busy(&self) -> bool {
        unsafe { self.read() & (1 << 24) != 0 }
    }
}

#[must_use]
pub struct Transfer<'a, C: ChannelControl, T> {
    channel_control: &'a C,
    result: T,
}

impl<'a, C: ChannelControl, T> Transfer<'a, C, T> {
    pub fn busy(&self) -> bool {
        self.channel_control.busy()
    }

    pub fn wait(self) -> T {
        while self.busy() {}
        self.result
    }

    pub fn consume(self) -> T {
        self.result
    }
}

pub enum MaybeTransfer<'a, C: ChannelControl, T> {
    Transfer(Transfer<'a, C, T>),
    Result(T),
}

impl<'a, C: ChannelControl, T> MaybeTransfer<'a, C, T> {
    pub fn maybe_wait(self) -> T {
        match self {
            MaybeTransfer::Transfer(t) => t.wait(),
            MaybeTransfer::Result(res) => res,
        }
    }

    pub fn consume(self) -> T {
        match self {
            MaybeTransfer::Transfer(t) => t.consume(),
            MaybeTransfer::Result(res) => res,
        }
    }
}

#[macro_export]
macro_rules! if_done {
    ($transfer:expr) => {{
        if !$transfer.busy() {
            Some($transfer.consume())
        } else {
            None
        }
    }};
}

// Methods for toggling DMA channels via dma::Control
macro_rules! toggle_fn {
    ($name:ident, $num:expr) => {
        pub(crate) fn $name(&mut self, enable: bool) -> &mut Self {
            let bit = (4 * $num) + 3;
            unsafe {
                self.update(|val| {
                    if enable {
                        val | (1 << bit)
                    } else {
                        val & !(1 << bit)
                    }
                })
            }
            self
        }
    };
}

impl dma::Control {
    toggle_fn!(mdec_in, 0);

    toggle_fn!(mdec_out, 1);

    toggle_fn!(gpu, 2);

    toggle_fn!(cdrom, 3);

    toggle_fn!(spu, 4);

    toggle_fn!(pio, 5);

    toggle_fn!(otc, 6);
}
