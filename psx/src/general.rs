use crate::cop0;
use crate::dma;
use crate::dma::{Channel, DPCR};
use crate::gpu::{GP1, GPUSTAT};
use crate::irq::{IMASK, IRQ, ISTAT};
use crate::timer::timer1::{CNT, MODE};
use crate::timer::{Source, SyncMode};
use crate::value::{Load, LoadMut};

/// Executes the given closure in a critical section and returns the result.
#[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
pub fn critical_section<F: FnOnce() -> R, R>(f: F) -> R {
    cop0::Status.load_mut().enter_critical_section().store();
    let r = f();
    cop0::Status.load_mut().exit_critical_section().store();
    r
}

/// Resets the GPU.
pub fn reset_graphics(gpu_dma: &mut dma::gpu::CHCR) {
    DPCR.skip_load()
        .disable_all()
        .enable(Channel::GPU)
        .enable(Channel::OTC)
        .store();
    gpu_dma.load_mut().chop(None).store();
    GP1.reset_gpu();
    IMASK.skip_load().disable_all().enable(IRQ::Vblank).store();
    MODE.skip_load()
        .enable_sync(false)
        .target_reset(false)
        .source(Source::Alternate)
        .sync_mode(SyncMode::FreeRun)
        .store();
}

/// Enables the display.
#[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
pub fn enable_display() {
    GP1.display_enable(true);
}

/// Waits for the next vertical blank.
#[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
pub fn vsync() -> u16 {
    ISTAT.load_mut().ack(IRQ::Vblank).store();
    ISTAT.wait(IRQ::Vblank);
    let mut counter = CNT;
    let mut old = counter.load_mut();
    let time = old.value.bits;
    old.value.bits = 0;
    old.store();
    time
}

/// Waits for the GPU to finish drawing.
#[cfg_attr(not(feature = "no_inline_hints"), inline(always))]
pub fn draw_sync() {
    while {
        let stat = GPUSTAT.load();
        !(stat.cmd_ready() && stat.dma_ready())
    } {}
}
