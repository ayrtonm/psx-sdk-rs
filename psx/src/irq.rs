//! Interrupt request types

/// An interrupt request
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IRQ {
    /// vertical blank interrupt request (NTSC = 60Hz, PAL = 50Hz)
    Vblank = 0,
    /// GPU interrupt requested via the GP0(1Fh) command
    GPU,
    /// CDROM interrupt request
    CDROM,
    /// DMA interrupt request
    DMA,
    /// Timer 0 (dot clock or sysclock)
    Timer0,
    /// Timer 1 (Hblank or sysclock)
    Timer1,
    /// Timer 2 (sysclock or fractional sysclock)
    Timer2,
    /// Controller and memory card byte received
    ControllerMemoryCard,
    /// Serial IO port
    SIO,
    /// Sound processing unit
    SPU,
    /// Secondary controller interrupt request
    ControllerPIO,
}
