//! CPU-side DMA channel routines.

use crate::mmio::dma;
use crate::mmio::register::{Read, Update, Write};

pub enum BlockSize {
    Single(u32),
    Multi { words: u16, blocks: u16 },
    LinkedList,
}

impl From<u32> for BlockSize {
    fn from(words: u32) -> Self {
        BlockSize::Single(words)
    }
}

impl From<(u16, u16)> for BlockSize {
    fn from((words, blocks): (u16, u16)) -> Self {
        BlockSize::Multi { words, blocks }
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

pub trait BaseAddress: Read + Write {
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
pub trait BlockControl: Read + Write {
    fn get(&self, sync_mode: SyncMode) -> Option<BlockSize> {
        let value = unsafe { self.read() };
        match sync_mode {
            SyncMode::Immediate => match value {
                0 => Some(BlockSize::Single(0x1_0000)),
                1..=0xFFFF => Some(BlockSize::Single(value)),
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
                0..=0xFFFF => words,
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
pub trait ChannelControl: Update {
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
    fn start<T: Copy>(&mut self, result: T) -> Transfer<Self, T> {
        unsafe {
            match self.sync_mode() {
                Some(SyncMode::Immediate) => self.update(|val| val | (1 << 24) | (1 << 28)),
                _ => self.update(|val| val | (1 << 24)),
            }
        }
        Transfer {
            control: self,
            result,
        }
    }
    fn busy(&self) -> bool {
        unsafe { self.read() & (1 << 24) != 0 }
    }
}

#[must_use]
pub struct Transfer<'a, C: ChannelControl + ?Sized, T: Copy> {
    control: &'a C,
    result: T,
}

impl<C: ChannelControl, T: Copy> Transfer<'_, C, T> {
    pub fn busy(&self) -> bool {
        self.control.busy()
    }

    pub fn wait(self) -> T {
        while self.busy() {}
        self.result
    }

    pub fn if_done(&self) -> Option<T> {
        if !self.busy() {
            Some(self.result)
        } else {
            None
        }
    }
}

macro_rules! enable_fn {
    ($name:ident, $bit:expr) => {
        pub fn $name(&mut self, enable: bool) {
            unsafe {
                self.update(|val| {
                    if enable {
                        val | (1 << $bit)
                    } else {
                        val & !(1 << $bit)
                    }
                })
            }
        }
    };
}
impl dma::Control {
    enable_fn!(gpu, 11);

    enable_fn!(otc, 27);
}
