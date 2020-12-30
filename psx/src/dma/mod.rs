use crate::value;
use crate::value::LoadMut;

/// DMA control register. Used to enable DMA channels and set priorities.
pub mod control;
/// Methods for using the Graphics Processing Unit DMA channel.
pub mod gpu;
/// DMA interrupt register. Used to enable and force DMA IRQs.
pub mod interrupt;
/// Methods for using the Ordering table clear DMA channel.
pub mod otc;

/// Representation of a DMA transfer.
pub mod transfer;

pub use control::DPCR;
pub use interrupt::DICR;
pub use transfer::Transfer;

/// A [DMA channel](http://problemkaputt.de/psx-spx.htm#dmachannels).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    /// DMA channel for RAM to Macroblock Decoder transfers.
    MDECin = 0,
    /// DMA channel for Macroblock Decoder to RAM transfers.
    MDECout,
    /// DMA channel for Graphics Processing Unit linked lists and image data.
    GPU,
    /// DMA channel for CDROM to RAM transfers.
    CDROM,
    /// DMA channel for RAM to Sound Processing Unit transfers.
    SPU,
    /// DMA channel for Expansion port transfers.
    PIO,
    /// DMA channel for clearing GPU ordering tables.
    OTC,
}

/// A DMA channel's block mode and size.
pub enum BlockMode {
    /// Transfers a single block. The number of words ranges from 1 to 0x1_0000
    /// words.
    Single(u32),
    /// Transfers `blocks` of `words` words each.
    Multi {
        /// Number of words in each block.
        words: u16,
        /// Number of blocks to transfer.
        blocks: u16,
    },
    /// Transfers in linked list mode.
    LinkedList,
}

impl From<u32> for BlockMode {
    #[cfg_attr(feature = "inline_hints", inline(always))]
    fn from(words: u32) -> BlockMode {
        BlockMode::Single(words)
    }
}

impl From<usize> for BlockMode {
    #[cfg_attr(feature = "inline_hints", inline(always))]
    fn from(words: usize) -> BlockMode {
        BlockMode::Single(words as u32)
    }
}

/// A DMA transfer's direction.
pub enum Direction {
    /// Transfer occurs from a peripheral to memory. Also used for clearing
    /// ordering tables.
    ToMemory = 0,
    /// Transfer occurs from memory to a peripheral.
    FromMemory,
}

/// A DMA transfer's memory address step.
pub enum Step {
    /// Transfer increments the address by 4 bytes in each step.
    Forward = 0,
    /// Transfer decrements the address by 4 bytes in each step.
    Backward,
}

/// A DMA transfer's chop settings.
pub struct Chop {
    /// The transfer's 3-bit DMA window size. DMA gets `1 << dma_window` clicks.
    pub dma_window: u32,
    /// The transfer's 3-bit CPU window size. CPU gets `1 << cpu_window` clicks.
    pub cpu_window: u32,
}

/// A DMA channel's transfer mode.
pub enum TransferMode {
    /// Starts immediately and transfers all at once.
    Immediate = 0,
    /// Sync blocks to DMA requests.
    Request,
    /// Transfers in linked list mode.
    LinkedList,
}

/// A marker for DMA base address registers.
pub trait BaseAddress: LoadMut<u32> {
    /// Gets the DMA channel's base address.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    fn get(&self) -> u32 {
        unsafe { self.read() }
    }

    /// Sets the DMA channel's base address.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    fn set(&mut self, addr: &u32) {
        unsafe { self.write(addr as *const u32 as u32) }
    }
}

/// A marker for DMA block control registers.
pub trait BlockControl: LoadMut<u32> {
    /// Gets the DMA channel's block size with the given `TransferMode`. Returns
    /// `None` if the register contains an invalid value.
    fn get(&self, transfer_mode: TransferMode) -> Option<BlockMode> {
        let bits = unsafe { self.read() };
        match transfer_mode {
            TransferMode::Immediate => match bits {
                0 => Some(0x1_0000u32.into()),
                1..=0xFFFF => Some(bits.into()),
                _ => None,
            },
            TransferMode::Request => Some(BlockMode::Multi {
                words: bits as u16,
                blocks: (bits >> 16) as u16,
            }),
            TransferMode::LinkedList => Some(BlockMode::LinkedList),
        }
    }

    /// Sets the DMA channel's block size.
    fn set<T>(&mut self, block_mode: T)
    where BlockMode: From<T> {
        let words = match block_mode.into() {
            BlockMode::Single(words) => match words {
                0..=0xFFFF => words as u32,
                0x1_0000 => 0,
                // TODO: add conditional runtime check and panic here
                _ => 0,
            },
            BlockMode::Multi { words, blocks } => words as u32 | ((blocks as u32) << 16),
            BlockMode::LinkedList => 0,
        };
        unsafe { self.write(words) }
    }
}

/// A marker for DMA channel control registers.
#[allow(missing_docs)]
pub trait ChannelControl: LoadMut<u32> {
    const DIRECTION: u32 = 0;
    const STEP: u32 = 1;
    const CHOP: u32 = 8;
    const TRANSFER_MODE: u32 = 9;
    const DMA_WIN: u32 = 16;
    const CPU_WIN: u32 = 20;
    const BUSY: u32 = 24;
}

/// A [`value::Value`] alias for DMA channel control registers.
pub type Value<'r, R> = value::Value<'r, u32, R>;
/// A [`value::MutValue`] alias for DMA channel control registers.
pub type MutValue<'r, R> = value::MutValue<'r, u32, R>;

impl<R: ChannelControl> Value<'_, R> {
    /// Gets the DMA channel's transfer mode. Returns `None` if the register
    /// contains an invalid value.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn transfer_mode(&self) -> Option<TransferMode> {
        match (self.bits >> R::TRANSFER_MODE) & 0b11 {
            0 => Some(TransferMode::Immediate),
            1 => Some(TransferMode::Request),
            2 => Some(TransferMode::LinkedList),
            _ => None,
        }
    }

    /// Checks if the DMA channel is busy.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn busy(&self) -> bool {
        self.contains(1 << R::BUSY)
    }
}

impl<'r, R: ChannelControl> MutValue<'r, R> {
    /// Sets the DMA channel's transfer direction.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn direction(self, direction: Direction) -> Self {
        self.clear(1).set(direction as u32)
    }

    /// Sets the DMA channel's transfer step.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn step(self, step: Step) -> Self {
        self.clear(1 << R::STEP).set((step as u32) << R::STEP)
    }

    /// Sets the DMA channel's chop settings.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn chop(self, chop: Option<Chop>) -> Self {
        match chop {
            Some(chop) => self
                .clear(chop.cpu_window << R::CPU_WIN | chop.dma_window << R::DMA_WIN)
                .set(1 << R::CHOP | chop.cpu_window << R::CPU_WIN | chop.dma_window << R::DMA_WIN),
            None => self.clear(1 << R::CHOP),
        }
    }

    /// Sets the DMA channel's transfer mode.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn transfer_mode(self, transfer_mode: TransferMode) -> Self {
        self.clear(0b11 << R::TRANSFER_MODE)
            .set((transfer_mode as u32) << R::TRANSFER_MODE)
    }

    /// Starts a DMA transfer, consuming the [`MutValue`] and giving the
    /// resulting [`Transfer`] shared access to the register.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn start<T>(self, result: T) -> Transfer<'r, T, R> {
        // TODO: Add bit 28 for transfer mode = 0.
        Transfer::new(self.set(1 << R::BUSY).take(), result)
    }

    /// Stops any ongoing DMA transfer for the given channel.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn stop(self) -> Self {
        self.clear(1 << R::BUSY)
    }
}
