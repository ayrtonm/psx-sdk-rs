use super::{Channel, Direction, Name};
use super::{Step, TransferMode};
use crate::hal::dma::{BlockControl, ChannelControl, MemoryAddress,
		      ty::BlockMode};
use crate::hal::{MutRegister, Mutable, Register, Shared, DPCR};

impl<MADR, BCR, CHCR, const NAME: Name> Channel<MADR, BCR, CHCR, NAME>
where
    MADR: MemoryAddress,
    BCR: BlockControl,
    CHCR: ChannelControl,
{
    pub fn enabled() -> bool {
        DPCR::<Shared>::load().enabled(NAME)
    }

    /// Enables the channel and returns its registers.
    pub fn new() -> Self {
        DPCR::<Mutable>::load().enable(NAME).store();
        Self::skip_enable()
    }

    pub fn reload(&mut self) {
        self.madr.reload();
        self.bcr.reload();
        self.chcr.reload();
    }

    /// Returns the channel's registers without enabling it.
    pub fn skip_enable() -> Self {
        // TODO: reconsider if this function should load MADR and BCR
        Channel {
            madr: unsafe { MADR::skip_load() },
            bcr: unsafe { BCR::skip_load() },
            chcr: CHCR::load(),
        }
    }

    pub fn split(self) -> (MADR, BCR, CHCR) {
        (self.madr, self.bcr, self.chcr)
    }

    /// Sends `buffer` to the channel all at once, without synchronizing. This
    /// will only work for DMA targets that don't require synchronizing, such
    /// as the CD-ROM controller.
    pub fn send<'b>(&mut self, buffer: &'b [u32]) {
	assert!(buffer.len() < 0x10000);
        self.chcr
            .set_direction(Direction::FromMemory)
            .set_step(Step::Forward)
            .set_mode(TransferMode::Immediate);
        self.madr.set_bits(buffer.as_ptr() as u32).store();
        self.bcr.set_block(buffer.len()).store();
        self.chcr.start().store().wait();
    }

    /// Sends `buffer` to the channel in chunks, with synchronization. This
    /// allows use with DMA targets that have limited receive buffers and take
    /// some time to process the received data, such as the GPU, SPU, and MDEC.
    pub fn send_chunked<'b, 'a: 'b>(&'a mut self, mut buffer: &'b [u32],
			    block_size: usize) ->
    ChunkedSendGuard<'b, BCR, CHCR>{
	assert!(block_size < 0x10000);
        self.chcr
            .set_direction(Direction::FromMemory)
            .set_step(Step::Forward)
            .set_mode(TransferMode::Request);
        self.madr.set_bits(buffer.as_ptr() as u32).store();
	if buffer.len() >= block_size {
	    // Transfer a whole number of blocks.
	    let num_blocks = buffer.len() / block_size;
	    assert!(num_blocks < 0x10000);
	    let block = BlockMode::Multi {
		words: block_size as u16,
		blocks: num_blocks as u16,
	    };
            self.bcr.set_block(block).store();
	    self.chcr.start().store();
	    // Calculate the number of words transferred *after* we start the
	    // DMA.
	    let transferred_words = num_blocks * block_size;
	    buffer = &buffer[transferred_words..];
	}
	ChunkedSendGuard {
	    bcr: &mut self.bcr,
	    chcr: &mut self.chcr,
	    remainder: buffer,
	}
    }
}

/// Synchronization primitive used by [`Channel::send_chunked`][1].
///
/// While a chunked transfer is in progress, neither the buffer being sent nor
/// the DMA channel it's being sent over can be accessed safely, but the CPU is
/// still free to perform other tasks. You can call [`busy()`][2] to poll for
/// completion:
///
/// ```
/// # fn update_music() {}
/// # let buffer: &[u32] = &[];
/// # use crate::dma;
/// let mut gpu_dma = dma::GPU::new();
/// let mut guard = gpu_dma.send_chunked(buffer, 16);
/// while guard.busy() {
///     update_music();
/// }
/// ```
///
/// ...or call [`finish()`][3] to wait until the transfer has concluded:
///
/// ```
/// # let buffer: &[u32] = &[];
/// # use crate::dma;
/// let mut gpu_dma = dma::GPU::new();
/// gpu_dma.send_chunked(buffer, 16).finish();
/// ```
///
/// `finish()` is automatically called when the guard is dropped, so if you
/// just end a block or function call with a call to `send_chunked()` and do
/// nothing with the guard, the transfer will be finished safely.
///
/// [1]: struct.Channel.html#method.send
/// [2]: #method.busy
/// [3]: #method.finish
pub struct ChunkedSendGuard<'a, BCR, CHCR>
where
    BCR: BlockControl,
    CHCR: ChannelControl,
{
    bcr: &'a mut BCR,
    chcr: &'a mut CHCR,
    remainder: &'a [u32],
}

impl<'a, BCR, CHCR> Drop for ChunkedSendGuard<'a, BCR, CHCR>
where
    BCR: BlockControl,
    CHCR: ChannelControl,
{
    fn drop(&mut self) {
	while self.busy() {}
    }
}

impl<'a, BCR, CHCR> ChunkedSendGuard<'a, BCR, CHCR>
where
    BCR: BlockControl,
    CHCR: ChannelControl,
{
    /// Returns true if the transfer is still in progress, false if it has
    /// completed.
    pub fn busy(&mut self) -> bool {
	self.chcr.reload();
	if self.remainder.len() > 0 && !self.chcr.busy() {
	    // MADR has been updated by the DMA process, so we don't need to
	    // massage it.
	    let block = BlockMode::Multi {
		words: self.remainder.len() as u16,
		blocks: 1,
	    };
	    self.bcr.set_block(block).store();
            self.chcr.start().store();
	    self.remainder = &[];
	}
	self.chcr.busy()
    }
    /// Waits until the transfer completes, then finishes it, relinquishing
    /// control over both the buffer being transferred and the channel it's
    /// being transferred to.
    ///
    /// Automatically called when this guard is dropped.
    pub fn finish(self) {
	// (our logic ends up in `Drop`)
    }
}
