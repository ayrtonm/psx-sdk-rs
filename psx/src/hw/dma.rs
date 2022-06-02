//! Direct memory access (DMA) control registers

use crate::dma::{BlockMode, Chop, Direction, Error, Step, TransferMode};
use crate::hw::{MemRegister, Register};

type Result<T> = core::result::Result<T, Error>;

const STEP: u32 = 1;
const CHOP: u32 = 8;
const TRANFER_MODE: u32 = 9;
const DMA_WIN: u32 = 16;
const CPU_WIN: u32 = 20;
const BUSY: u32 = 24;
const TRIGGER: u32 = 28;

/// Only the lower 24 bits are stored in the address registers.
const ADDRESS_MASK: u32 = 0x00FF_FFFF;

/// The name of a DMA channel
#[derive(Debug)]
pub enum Name {
    /// RAM to Macroblock decoder DMA channel
    MDECIn = 0,
    /// Macroblock decoder to RAM DMA channel
    MDECOut,
    /// GPU DMA channel
    GPU,
    /// CD-ROM DMA channel
    CDROM,
    /// SPU DMA channel
    SPU,
    /// Expansion port DMA channel
    PIO,
    /// Ordering table clear DMA channel
    OTC,
}

/// RAM to Macroblock decode DMA channel.
pub mod mdec_in {
    use super::{BlockControl, ChannelControl, MemoryAddress};
    use crate::hw::dma::Name;
    use crate::hw::MemRegister;

    /// DMA channel address register.
    pub type Address = MemRegister<u32, 0x1F80_1080>;
    impl MemoryAddress for Address {}

    /// DMA channel block register.
    pub type Block = MemRegister<u32, 0x1F80_1084>;
    impl BlockControl for Block {}

    /// DMA channel control register.
    pub type Control = MemRegister<u32, 0x1F80_1088>;
    impl ChannelControl for Control {
        const NAME: Name = Name::MDECIn;
    }
}

/// Macroblock decoder to RAM DMA channel
pub mod mdec_out {
    use super::{BlockControl, ChannelControl, MemoryAddress};
    use crate::hw::dma::Name;
    use crate::hw::MemRegister;

    /// DMA channel address register.
    pub type Address = MemRegister<u32, 0x1F80_1090>;
    impl MemoryAddress for Address {}

    /// DMA channel block register.
    pub type Block = MemRegister<u32, 0x1F80_1094>;
    impl BlockControl for Block {}

    /// DMA channel control register.
    pub type Control = MemRegister<u32, 0x1F80_1098>;
    impl ChannelControl for Control {
        const NAME: Name = Name::MDECOut;
    }
}

/// GPU DMA channel
pub mod gpu {
    use super::{BlockControl, ChannelControl, MemoryAddress};
    use crate::hw::dma::Name;
    use crate::hw::MemRegister;

    /// DMA channel address register.
    pub type Address = MemRegister<u32, 0x1F80_10A0>;
    impl MemoryAddress for Address {}

    /// DMA channel block register.
    pub type Block = MemRegister<u32, 0x1F80_10A4>;
    impl BlockControl for Block {}

    /// DMA channel control register.
    pub type Control = MemRegister<u32, 0x1F80_10A8>;
    impl ChannelControl for Control {
        const NAME: Name = Name::GPU;
    }
}

/// CD-ROM DMA channel
pub mod cdrom {
    use super::{BlockControl, ChannelControl, MemoryAddress};
    use crate::hw::dma::Name;
    use crate::hw::MemRegister;

    /// DMA channel address register.
    pub type Address = MemRegister<u32, 0x1F80_10B0>;
    impl MemoryAddress for Address {}

    /// DMA channel block register.
    pub type Block = MemRegister<u32, 0x1F80_10B4>;
    impl BlockControl for Block {}

    /// DMA channel control register.
    pub type Control = MemRegister<u32, 0x1F80_10B8>;
    impl ChannelControl for Control {
        const NAME: Name = Name::CDROM;
    }
}

/// SPU DMA channel
pub mod spu {
    use super::{BlockControl, ChannelControl, MemoryAddress};
    use crate::hw::dma::Name;
    use crate::hw::MemRegister;

    /// DMA channel address register.
    pub type Address = MemRegister<u32, 0x1F80_10C0>;
    impl MemoryAddress for Address {}

    /// DMA channel block register.
    pub type Block = MemRegister<u32, 0x1F80_10C4>;
    impl BlockControl for Block {}

    /// DMA channel control register.
    pub type Control = MemRegister<u32, 0x1F80_10C8>;
    impl ChannelControl for Control {
        const NAME: Name = Name::SPU;
    }
}

/// Expansion port DMA channel
pub mod pio {
    use super::{BlockControl, ChannelControl, MemoryAddress};
    use crate::hw::dma::Name;
    use crate::hw::MemRegister;

    /// DMA channel address register.
    pub type Address = MemRegister<u32, 0x1F80_10D0>;
    impl MemoryAddress for Address {}

    /// DMA channel block register.
    pub type Block = MemRegister<u32, 0x1F80_10D4>;
    impl BlockControl for Block {}

    /// DMA channel control register.
    pub type Control = MemRegister<u32, 0x1F80_10D8>;
    impl ChannelControl for Control {
        const NAME: Name = Name::PIO;
    }
}

/// Ordering table clear DMA channel
pub mod otc {
    use super::{BlockControl, ChannelControl, MemoryAddress};
    use crate::hw::dma::Name;
    use crate::hw::MemRegister;

    /// DMA channel address register.
    pub type Address = MemRegister<u32, 0x1F80_10E0>;
    impl MemoryAddress for Address {}

    /// DMA channel block register.
    pub type Block = MemRegister<u32, 0x1F80_10E4>;
    impl BlockControl for Block {}

    /// DMA channel control register.
    pub type Control = MemRegister<u32, 0x1F80_10E8>;
    impl ChannelControl for Control {
        const NAME: Name = Name::OTC;
    }
}

/// DMA Control register
pub type GlobalControl = MemRegister<u32, 0x1F80_10F0>;

/// DMA Interrupt register
pub type Interrupt = MemRegister<u32, 0x1F80_10F4>;

/// An address register for a DMA channel.
pub trait MemoryAddress: Register<u32> {
    /// Get a pointer to the channel's start address.
    fn get_address(&self) -> *const u32 {
        (self.as_ref() & ADDRESS_MASK) as *const u32
    }

    /// Sets the channel's start address.
    fn set_address(&mut self, ptr: &u32) -> &mut Self {
        let val = (ptr as *const u32 as u32) & ADDRESS_MASK;
        self.assign(val)
    }
}

/// A block size control register for a DMA channel.
pub trait BlockControl: Register<u32> {
    /// Gets the block mode assuming the DMA channel is in the specified
    /// transfer mode.
    fn get_block(&self, transfer_mode: TransferMode) -> Option<BlockMode> {
        match transfer_mode {
            TransferMode::Immediate => match self.as_ref() {
                0 => Some(BlockMode::Single(0x1_0000u32)),
                1..=0xFFFF => Some(BlockMode::Single(*self.as_ref())),
                _ => None,
            },
            TransferMode::Request => Some(BlockMode::Multi {
                words: *self.as_ref() as u16,
                blocks: (self.as_ref() >> 16) as u16,
            }),
            TransferMode::LinkedList => Some(BlockMode::LinkedList),
        }
    }

    /// Sets the block mode for the DMA channel, returning `None` if the block
    /// size is too large.
    fn set_block<B>(&mut self, block_mode: B) -> Result<&mut Self>
    where BlockMode: From<B> {
        match block_mode.into() {
            BlockMode::Single(words) => {
                let words = match words {
                    0..=0xFFFF => words,
                    0x1_0000 => 0,
                    _ => return Err(Error::OversizedBlock),
                };
                Ok(self.assign(words))
            },
            BlockMode::Multi { words, blocks } => {
                let value = words as u32 | ((blocks as u32) << 16);
                Ok(self.assign(value))
            },
            BlockMode::LinkedList => Ok(self),
        }
    }
}

/// A channel control register for a DMA channel.
pub trait ChannelControl: Register<u32> {
    /// The name of the DMA channel.
    const NAME: Name;

    /// Wait until the DMA channel is not busy.
    fn wait(&mut self) -> &mut Self {
        while self.busy() {
            self.load();
        }
        self
    }

    /// Checks if the DMA channel is busy.
    fn busy(&self) -> bool {
        self.all_set(1 << BUSY)
    }

    /// Gets the DMA channel transfer mode, returning `None` for invalid values.
    fn get_mode(&self) -> Option<TransferMode> {
        match (self.as_ref() >> TRANFER_MODE) & 0b11 {
            0 => Some(TransferMode::Immediate),
            1 => Some(TransferMode::Request),
            2 => Some(TransferMode::LinkedList),
            _ => None,
        }
    }

    /// Checks the direction of the DMA channel.
    fn get_direction(&self) -> Direction {
        if self.all_set(1) {
            Direction::FromMemory
        } else {
            Direction::ToMemory
        }
    }

    /// Checks the memory address step of the DMA channel.
    fn get_step(&self) -> Step {
        if self.all_set(1 << STEP) {
            Step::Backward
        } else {
            Step::Forward
        }
    }

    /// Checks the channel's CPU/DMA window sizes, returning `None` if chopping
    /// is disabled or if the value is invalid.
    fn get_chop(&self) -> Option<Chop> {
        if self.all_set(1 << CHOP) {
            let cpu_window = self.as_ref() >> CPU_WIN;
            let dma_window = self.as_ref() >> DMA_WIN;
            if cpu_window & 0b111 != 0 {
                return None
            }
            if dma_window & 0b111 != 0 {
                return None
            }
            Some(Chop {
                cpu_window,
                dma_window,
            })
        } else {
            None
        }
    }

    /// Sets the direction of the DMA channel.
    fn set_direction(&mut self, direction: Direction) -> &mut Self {
        self.clear_bits(1).set_bits(direction as u32)
    }

    /// Sets the memory address step of the DMA channel.
    fn set_step(&mut self, step: Step) -> &mut Self {
        self.clear_bits(1 << STEP).set_bits((step as u32) << STEP)
    }

    /// Enables chopping and sets the channel's CPU/DMA window sizes. Chopping
    /// is disabled if `chop` is `None`. Returns `None` if the chopping windows
    /// are invalid.
    fn set_chop(&mut self, chop: Option<Chop>) -> Option<&mut Self> {
        match chop {
            Some(chop) => {
                if chop.cpu_window & !0b111 != 0 {
                    return None
                }
                if chop.dma_window & !0b111 != 0 {
                    return None
                }
                Some(
                    self.clear_bits(0b111 << CPU_WIN | 0b111 << DMA_WIN)
                        .set_bits(
                            1 << CHOP | chop.cpu_window << CPU_WIN | chop.dma_window << DMA_WIN,
                        ),
                )
            },
            None => Some(self.clear_bits(1 << CHOP)),
        }
    }

    /// Sets the DMA channel's transfer mode.
    fn set_mode(&mut self, mode: TransferMode) -> &mut Self {
        self.clear_bits(0b11 << TRANFER_MODE)
            .set_bits((mode as u32) << TRANFER_MODE)
    }

    /// Starts a DMA transfer.
    fn start(&mut self) -> &mut Self {
        if let Some(TransferMode::Immediate) = self.get_mode() {
            self.set_bits(1 << TRIGGER);
        }
        self.set_bits(1 << BUSY)
    }

    /// Stops an ongoing DMA transfer.
    fn stop(&mut self) -> &mut Self {
        self.clear_bits(1 << BUSY)
    }
}

impl GlobalControl {
    const fn enable_bit(channel: Name) -> u32 {
        let bit = (channel as u32 * 4) + 3;
        1 << bit
    }

    const fn all_channels() -> u32 {
        Self::enable_bit(Name::MDECIn) |
            Self::enable_bit(Name::MDECOut) |
            Self::enable_bit(Name::GPU) |
            Self::enable_bit(Name::CDROM) |
            Self::enable_bit(Name::SPU) |
            Self::enable_bit(Name::PIO) |
            Self::enable_bit(Name::OTC)
    }

    /// Checks if the DMA `channel` is enabled.
    pub fn enabled(&self, channel: Name) -> bool {
        self.all_set(Self::enable_bit(channel))
    }

    /// Enables the DMA `channel`.
    pub fn enable(&mut self, channel: Name) -> &mut Self {
        self.set_bits(Self::enable_bit(channel))
    }

    /// Disables the DMA `channel`.
    pub fn disable(&mut self, channel: Name) -> &mut Self {
        self.clear_bits(Self::enable_bit(channel))
    }

    /// Enables all DMA channels.
    pub fn enable_all(&mut self) -> &mut Self {
        self.set_bits(Self::all_channels())
    }

    /// Disables all DMA channels.
    pub fn disable_all(&mut self) -> &mut Self {
        self.clear_bits(Self::all_channels())
    }
}
