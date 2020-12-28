/// IRQ mask register.
pub mod mask;
/// IRQ status register.
pub mod stat;

pub use mask::IMASK;
pub use stat::ISTAT;

/// An interrupt request.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IRQ {
    /// Vertical blank interrupt.
    Vblank = 0,
    /// GPU interrupt request.
    GPU,
    CDROM,
    DMA,
    Timer0,
    Timer1,
    Timer2,
    Controller1,
    SIO,
    SPU,
    Controller2,
}
