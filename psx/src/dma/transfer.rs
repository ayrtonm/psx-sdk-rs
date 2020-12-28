use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use super::ChannelControl;

use crate::value::Load;

/// Represents a DMA transfer with a precomputed result that can only be
/// accessed after the transfer ends.
#[must_use]
pub struct Transfer<'r, T, R: ChannelControl> {
    reg: &'r R,
    result: T,
}

impl<'r, T, R: ChannelControl> Transfer<'r, T, R> {
    /// Creates a new DMA transfer.
    #[inline(always)]
    pub fn new(reg: &'r R, result: T) -> Self {
        Transfer { reg, result }
    }

    /// Waits until the DMA transfer ends then returns the result.
    #[inline(always)]
    pub fn wait(self) -> T {
        while self.reg.load().busy() {}
        self.result
    }

    /// Immutably borrows the register used to start the transfer.
    #[inline(always)]
    pub fn borrow(&self) -> &R {
        self.reg
    }
}

impl<'r, T: Copy, R: ChannelControl> Future for Transfer<'r, T, R> {
    type Output = T;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.reg.load().busy() {
            Poll::Pending
        } else {
            Poll::Ready(self.result)
        }
    }
}
