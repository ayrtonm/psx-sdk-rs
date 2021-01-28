#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IRQ {
    Vblank = 0,
    GPU,
    CDROM,
    DMA,
    Timer0,
    Timer1,
    Timer2,
    ControllerMemoryCard,
    SIO,
    SPU,
    ControllerMisc,
}

pub fn free<F: FnOnce() -> R, R>(_f: F) -> R {
    todo!("");
}
