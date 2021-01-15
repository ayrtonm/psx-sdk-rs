use crate::hal::dma::{ChannelName, Direction, MutBlockControl, MutChannelControl,
                      MutMemoryAddress, Step, TransferMode};
use crate::hal::DPCR;
use crate::hal::{MutRegister, Mutable};
use crate::hal::{D0_BCR, D0_CHCR, D0_MADR};
use crate::hal::{D1_BCR, D1_CHCR, D1_MADR};
use crate::hal::{D2_BCR, D2_CHCR, D2_MADR};
use crate::hal::{D3_BCR, D3_CHCR, D3_MADR};
use crate::hal::{D4_BCR, D4_CHCR, D4_MADR};
use crate::hal::{D5_BCR, D5_CHCR, D5_MADR};
use crate::hal::{D6_BCR, D6_CHCR, D6_MADR};

pub struct Channel<MADR: MutMemoryAddress, BCR: MutBlockControl, CHCR: MutChannelControl> {
    madr: MADR,
    bcr: BCR,
    chcr: CHCR,
}

pub type MDECIn = Channel<D0_MADR<Mutable>, D0_BCR<Mutable>, D0_CHCR<Mutable>>;
pub type MDECOut = Channel<D1_MADR<Mutable>, D1_BCR<Mutable>, D1_CHCR<Mutable>>;
pub type GPU = Channel<D2_MADR<Mutable>, D2_BCR<Mutable>, D2_CHCR<Mutable>>;
pub type CDROM = Channel<D3_MADR<Mutable>, D3_BCR<Mutable>, D3_CHCR<Mutable>>;
pub type SPU = Channel<D4_MADR<Mutable>, D4_BCR<Mutable>, D4_CHCR<Mutable>>;
pub type PIO = Channel<D5_MADR<Mutable>, D5_BCR<Mutable>, D5_CHCR<Mutable>>;
pub type OTC = Channel<D6_MADR<Mutable>, D6_BCR<Mutable>, D6_CHCR<Mutable>>;

#[must_use]
pub struct Transfer<
    't,
    'c,
    T: ?Sized,
    MADR: MutMemoryAddress,
    BCR: MutBlockControl,
    CHCR: MutChannelControl,
> {
    data: &'t T,
    channel: &'c mut Channel<MADR, BCR, CHCR>,
}

impl<'t, 'c, T: ?Sized, MADR: MutMemoryAddress, BCR: MutBlockControl, CHCR: MutChannelControl>
    Transfer<'t, 'c, T, MADR, BCR, CHCR>
{
    pub fn wait(self) {}
}

impl<MADR: MutMemoryAddress, BCR: MutBlockControl, CHCR: MutChannelControl>
    Channel<MADR, BCR, CHCR>
{
    pub fn new_enabled() -> Self {
        Self {
            madr: MADR::skip_load(),
            bcr: BCR::skip_load(),
            chcr: CHCR::skip_load(),
        }
    }

    pub fn prepare(&mut self) {
        self.chcr
            .set_direction(Direction::FromMemory)
            .set_step(Step::Forward)
            .set_mode(TransferMode::Immediate);
    }

    pub fn send<'t>(&mut self, block: &'a [u32]) -> Transfer<'a, [u32], &mut Self> {
        self.madr.set(block.as_ptr() as u32).store();
        self.bcr.set_block(block.len()).store();
        self.chcr.start().store();
        Transfer {
            data: block,
            channel: self,
        }
    }
}

impl GPU {
    const NAME: ChannelName = ChannelName::GPU;
    pub fn new() -> Self {
        DPCR::load_mut().enable(Self::NAME).store();
        Self::new_enabled()
    }
}

impl OTC {
    const NAME: ChannelName = ChannelName::OTC;
    pub fn new() -> Self {
        DPCR::load_mut().enable(Self::NAME).store();
        Self::new_enabled()
    }
}
