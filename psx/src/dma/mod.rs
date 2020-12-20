//! CPU-side DMA channel routines.
use crate::mmio::register::{Read, Write};

pub enum BlockSize {
    Single(usize),
    Multi { words: u16, blocks: u16 },
    LinkedList,
}

impl From<usize> for BlockSize {
    #[inline(always)]
    fn from(words: usize) -> Self {
        BlockSize::Single(words)
    }
}
impl From<u32> for BlockSize {
    #[inline(always)]
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
    pub dma: u32,
    pub cpu: u32,
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
        //#[cfg(feature = "pretty_panic")]
        //{
        //    if address >> 24 != 0 {
        //        panic!("got here4");
        //    }
        //}
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
                    #[cfg(feature = "pretty_panic")]
                    {
                        panic!("Number of words can't exceed 0x1_0000");
                    }
                    #[cfg(not(feature = "pretty_panic"))]
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

mod control {
    use crate::mmio::dma;

    impl_mut_value!(dma::Control);

    // Methods for toggling DMA channels via dma::Control
    macro_rules! toggle_fn {
        ($name:ident, $num:expr) => {
            #[inline(always)]
            pub fn $name(mut self, enable: bool) -> Self {
                let bit = (4 * $num) + 3;
                if enable {
                    self.value.bits |= 1 << bit;
                } else {
                    self.value.bits &= !(1 << bit);
                }
                self
            }
        };
    }

    impl<'a> MutValue<'a> {
        toggle_fn!(mdec_in, 0);

        toggle_fn!(mdec_out, 1);

        toggle_fn!(gpu, 2);

        toggle_fn!(cdrom, 3);

        toggle_fn!(spu, 4);

        toggle_fn!(pio, 5);

        toggle_fn!(otc, 6);
    }
}

macro_rules! impl_dma_channel_control {
    ($reg:path) => {
        impl $reg {
            #[inline(always)]
            pub fn busy(&self) -> bool {
                use crate::mmio::register::Read;
                unsafe { self.read() & (1 << 24) != 0 }
            }
        }

        impl Value {
            #[inline(always)]
            pub fn current_sync_mode(&self) -> Option<SyncMode> {
                match (self.bits >> 9) & 0b11 {
                    0 => Some(SyncMode::Immediate),
                    1 => Some(SyncMode::Request),
                    2 => Some(SyncMode::LinkedList),
                    _ => None,
                }
            }
        }

        impl<'a> MutValue<'a> {
            #[inline(always)]
            pub fn direction(mut self, direction: Direction) -> Self {
                self.value.bits &= !1;
                self.value.bits |= direction as u32;
                self
            }

            #[inline(always)]
            pub fn step(mut self, step: Step) -> Self {
                self.value.bits &= !0b10;
                self.value.bits |= (step as u32) << 1;
                self
            }

            #[inline(always)]
            pub fn chop(mut self, chop: Option<Chop>) -> Self {
                match chop {
                    Some(chop) => {
                        #[cfg(feature = "pretty_panic")]
                        {
                            if chop.dma > 0b111 || chop.cpu > 0b111 {
                                panic!("DMA chopping windows are limited to 3 bits");
                            }
                        }
                        self.value.bits |= (1 << 8) | (chop.dma << 16) | (chop.cpu << 20);
                    },
                    None => {
                        self.value.bits &= !(1 << 8);
                    },
                }
                self
            }

            #[inline(always)]
            pub fn sync_mode(mut self, sync_mode: SyncMode) -> Self {
                self.value.bits &= !(0b11 << 9);
                self.value.bits |= (sync_mode as u32) << 9;
                self
            }

            #[inline(always)]
            pub fn start<T>(mut self, result: T) -> Transfer<'a, T> {
                self.value.bits |= match self.current_sync_mode() {
                    Some(SyncMode::Immediate) => (1 << 24) | (1 << 28),
                    _ => 1 << 24,
                };
                Transfer {
                    channel_control: self.take(),
                    result,
                }
            }
        }
        #[must_use]
        pub struct Transfer<'a, T> {
            channel_control: &'a $reg,
            result: T,
        }

        impl<'a, T> Transfer<'a, T> {
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

        pub enum MaybeTransfer<'a, T> {
            Transfer(Transfer<'a, T>),
            Result(T),
        }

        impl<'a, T> MaybeTransfer<'a, T> {
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
    };
}

macro_rules! value_mod {
    ($name:ident) => {
        pub mod $name {
            use super::{Chop, Direction, Step, SyncMode};
            use crate::mmio::dma;

            impl_mut_value!(dma::$name::ChannelControl);
            impl_dma_channel_control!(dma::$name::ChannelControl);
        }
    };
}

pub mod gpu;
pub mod otc;
value_mod!(mdec_in);
value_mod!(mdec_out);
value_mod!(cdrom);
value_mod!(spu);
value_mod!(pio);
