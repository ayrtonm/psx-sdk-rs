//! DMA channel routines

use crate::hal::dma::{BlockControl, ChannelControl, MemoryAddress};
use crate::hal::Mutable;
use crate::hal::{D0_BCR, D0_CHCR, D0_MADR};
use crate::hal::{D1_BCR, D1_CHCR, D1_MADR};
use crate::hal::{D2_BCR, D2_CHCR, D2_MADR};
use crate::hal::{D3_BCR, D3_CHCR, D3_MADR};
use crate::hal::{D4_BCR, D4_CHCR, D4_MADR};
use crate::hal::{D5_BCR, D5_CHCR, D5_MADR};
use crate::hal::{D6_BCR, D6_CHCR, D6_MADR};

pub use crate::hal::dma::ty::{BlockMode, Chop, Direction, Name, Step, TransferMode};

mod channel;
mod gpu;

pub use channel::ChunkedSendGuard;

/// DMA channel registers
pub struct Channel<MADR: MemoryAddress, BCR: BlockControl, CHCR: ChannelControl, const NAME: Name> {
    madr: MADR,
    bcr: BCR,
    chcr: CHCR,
}

/// DMA channel to transfer from RAM to the macroblock decoder
pub type MDECIn = Channel<D0_MADR<Mutable>, D0_BCR<Mutable>, D0_CHCR<Mutable>, { Name::MDECIn }>;
/// DMA channel to transfer from the macroblock decoder to RAM
pub type MDECOut = Channel<D1_MADR<Mutable>, D1_BCR<Mutable>, D1_CHCR<Mutable>, { Name::MDECOut }>;
/// DMA channel for GPU lists and image data
pub type GPU = Channel<D2_MADR<Mutable>, D2_BCR<Mutable>, D2_CHCR<Mutable>, { Name::GPU }>;
/// DMA channel to transfer CDROM data to RAM
pub type CDROM = Channel<D3_MADR<Mutable>, D3_BCR<Mutable>, D3_CHCR<Mutable>, { Name::CDROM }>;
pub type SPU = Channel<D4_MADR<Mutable>, D4_BCR<Mutable>, D4_CHCR<Mutable>, { Name::SPU }>;
pub type PIO = Channel<D5_MADR<Mutable>, D5_BCR<Mutable>, D5_CHCR<Mutable>, { Name::PIO }>;
pub type OTC = Channel<D6_MADR<Mutable>, D6_BCR<Mutable>, D6_CHCR<Mutable>, { Name::OTC }>;
