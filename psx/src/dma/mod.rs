use crate::value;
use crate::value::LoadMut;
use transfer::Transfer;

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

/// A [DMA channel](http://problemkaputt.de/psx-spx.htm#dmachannels).
#[repr(u32)]
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
    fn from(words: u32) -> BlockMode {
        BlockMode::Single(words)
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

/// A DMA channel's sync mode.
pub enum SyncMode {
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
    #[inline(always)]
    fn get(&self) -> u32 {
        unsafe { self.read() }
    }

    /// Sets the DMA channel's base address.
    #[inline(always)]
    fn set(&mut self, addr: u32) {
        unsafe { self.write(addr) }
    }
}

/// A marker for DMA block control registers.
pub trait BlockControl: LoadMut<u32> {
    /// Gets the DMA channel's block size with the given `SyncMode`. Returns
    /// `None` if the register contains an invalid value.
    fn get(&self, sync_mode: SyncMode) -> Option<BlockMode> {
        let bits = unsafe { self.read() };
        match sync_mode {
            SyncMode::Immediate => match bits {
                0 => Some(0x1_0000.into()),
                1..=0xFFFF => Some(bits.into()),
                _ => None,
            },
            SyncMode::Request => Some(BlockMode::Multi {
                words: bits as u16,
                blocks: (bits >> 16) as u16,
            }),
            SyncMode::LinkedList => Some(BlockMode::LinkedList),
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
    const DIRECTION: u32 = 1 << 0;
    const STEP: u32 = 1;
    const CHOP: u32 = 8;
    const SYNC_MODE: u32 = 9;
    const DMA_WIN: u32 = 16;
    const CPU_WIN: u32 = 20;
    const BUSY: u32 = 28;

    #[inline(always)]
    fn busy(&self) -> bool {
        unsafe { self.read() & (1 << Self::BUSY) != 0 }
    }
}

/// A [`value::Value`] alias for DMA channel control registers.
pub type Value<'r, R> = value::Value<'r, u32, R>;
/// A [`value::MutValue`] alias for DMA channel control registers.
pub type MutValue<'r, R> = value::MutValue<'r, u32, R>;

impl<R: ChannelControl> Value<'_, R> {
    /// Gets the DMA channel's sync mode. Returns `None` if the register
    /// contains an invalid value.
    #[inline(always)]
    pub fn sync_mode(&self) -> Option<SyncMode> {
        match (self.bits >> R::SYNC_MODE) & 0b11 {
            0 => Some(SyncMode::Immediate),
            1 => Some(SyncMode::Request),
            2 => Some(SyncMode::LinkedList),
            _ => None,
        }
    }
}

impl<'r, R: ChannelControl> MutValue<'r, R> {
    /// Sets the DMA channel's transfer direction.
    #[inline(always)]
    pub fn direction(self, direction: Direction) -> Self {
        self.clear(1).set(direction as u32)
    }

    /// Sets the DMA channel's transfer step.
    #[inline(always)]
    pub fn step(self, step: Step) -> Self {
        self.clear(1 << R::STEP).set((step as u32) << R::STEP)
    }

    /// Sets the DMA channel's chop settings.
    #[inline(always)]
    pub fn chop(self, chop: Option<Chop>) -> Self {
        match chop {
            Some(chop) => self
                .set(1 << R::CHOP | chop.cpu_window << R::CPU_WIN | chop.dma_window << R::DMA_WIN),
            None => self.clear(1 << R::CHOP),
        }
    }

    /// Sets the DMA channel's sync mode.
    #[inline(always)]
    pub fn sync_mode(self, sync_mode: SyncMode) -> Self {
        self.clear(0b11 << R::SYNC_MODE)
            .set((sync_mode as u32) << R::SYNC_MODE)
    }

    /// Starts a DMA transfer, consuming the [`MutValue`] and giving the
    /// resulting [`Transfer`] shared access to the register.
    #[inline(always)]
    pub fn start<T>(self, result: T) -> Transfer<'r, T, R> {
        Transfer::new(self.take(), result)
    }
}
