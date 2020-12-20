use core::iter;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IRQ {
    Vblank = 0,
    GPU,
    CDROM,
    DMA,
    Timer0,
    Timer1,
    Timer2,
    Controller,
    SIO,
    SPU,
    Controller2,
}

pub(super) const ALL_IRQS: [IRQ; 11] = [
    IRQ::Vblank,
    IRQ::GPU,
    IRQ::CDROM,
    IRQ::DMA,
    IRQ::Timer0,
    IRQ::Timer1,
    IRQ::Timer2,
    IRQ::Controller,
    IRQ::SIO,
    IRQ::SPU,
    IRQ::Controller2,
];

impl IntoIterator for IRQ {
    type Item = IRQ;

    type IntoIter = impl Iterator<Item = IRQ>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}
