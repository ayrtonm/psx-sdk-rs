use crate::cop0;
use crate::dma::{Channel, DPCR};
use crate::gpu::GP1;
use crate::irq::{IMASK, IRQ, ISTAT};
use crate::value::LoadMut;

/// Executes the given closure in a critical section and returns the result.
#[inline(always)]
pub fn critical_section<F: FnOnce() -> R, R>(f: F) -> R {
    cop0::Status.load_mut().enter_critical_section().store();
    let r = f();
    cop0::Status.load_mut().exit_critical_section().store();
    r
}

/// Resets the GPU.
pub fn reset_graphics() {
    DPCR.skip_load()
        .disable_all()
        .enable(Channel::GPU)
        .enable(Channel::OTC)
        .store();
    GP1.reset_gpu();
    IMASK.skip_load().disable_all().enable(IRQ::Vblank).store();
}

/// Enables the display.
pub fn enable_display() {
    GP1.display_enable(true);
}

/// Waits for the next vertical blank.
pub fn vsync() {
    ISTAT.load_mut().ack(IRQ::Vblank).store();
    ISTAT.wait(IRQ::Vblank);
}
