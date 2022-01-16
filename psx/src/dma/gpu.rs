use crate::dma::{Channel, Direction, LinkedList};
use crate::hw::dma::gpu::{Address, Block, Control};
use crate::hw::dma::ChannelControl;

/// The DMA channel for GPU transfers
pub struct GPU(Channel<Address, Block, Control>);

impl GPU {
    /// Initialize the GPU DMA channel.
    pub fn new() -> Self {
        let mut channel = Channel::<Address, Block, Control>::new();
        // Set the channel direction to a reasonable default, but intentionally
        // avoid defer the store until the transfer is initiated to avoid an
        // unnecessary write.
        channel.control.set_direction(Direction::FromMemory);
        GPU(channel)
    }

    /// The channel control register for the DMA channel.
    pub fn control(&mut self) -> &mut Control {
        &mut self.0.control
    }

    pub fn send_list<L: LinkedList + ?Sized>(&mut self, list: &L) {
        self.send_list_and(list, || ())
    }

    /// Sends a buffer through a DMA channel in single-block mode and call `f`
    /// while the transfer completes.
    ///
    /// This blocks if the function `f` returns before the transfer completes.
    /// Returns `f`'s return value or `None` if the buffer is too large.
    pub fn send_list_and<L: LinkedList + ?Sized, F: FnOnce() -> R, R>(
        &mut self, list: &L, f: F,
    ) -> R {
        self.0.send_list_and(list, f)
    }
}
