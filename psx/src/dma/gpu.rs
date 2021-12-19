use crate::dma::{Channel, LinkedList};
use crate::hw::dma::gpu::{Address, Block, Control};
use crate::Result;

/// The DMA channel for GPU transfers
pub struct GPU(Channel<Address, Block, Control>);

impl GPU {
    pub fn new() -> Self {
        GPU(Channel::new())
    }

    /// The channel control register for the DMA channel.
    pub fn control(&mut self) -> &mut Control {
        &mut self.0.control
    }

    pub fn send_list<L: LinkedList>(&mut self, list: &L) -> Result<()> {
        self.send_list_and(list, || ())
    }

    /// Sends a buffer through a DMA channel in single-block mode and call `f`
    /// while the transfer completes.
    ///
    /// This blocks if the function `f` returns before the transfer completes.
    /// Returns `f`'s return value or `None` if the buffer is too large.
    pub fn send_list_and<L: LinkedList, F: FnOnce() -> R, R>(
        &mut self, list: &L, f: F,
    ) -> Result<R> {
        self.0.send_list_and(list, f)
    }
}
