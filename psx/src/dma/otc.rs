use crate::dma::Channel;
use crate::hw::dma::otc::{Address, Block, Control};
use crate::Result;

pub struct OTC(Channel<Address, Block, Control>);

impl OTC {
    pub fn new() -> Self {
        OTC(Channel::new())
    }

    pub fn control(&mut self) -> &mut Control {
        &mut self.0.control
    }

    pub fn send(&mut self, block: &[u32]) -> Result<()> {
        self.0.send_and(block, || ())
    }

    pub fn send_and<F: FnOnce() -> R, R>(&mut self, block: &[u32], f: F) -> Result<R> {
        self.0.send_and(block, f)
    }
}
