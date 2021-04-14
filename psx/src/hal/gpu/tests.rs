#![cfg(test)]
use super::ty::DMAMode;
use crate::hal::{Register, GP0, GP1, GPUSTAT};

#[test_case]
fn reset_gpu() {
    GP1.reset_gpu();
    let stat = GPUSTAT::load().bits_no_parity();
    assert!(stat == 0x1480_2000);
}

#[test_case]
fn ack_irq() {
    let mut stat = GPUSTAT::load();
    assert!(!stat.irq_pending());
    GP0.interrupt_request();
    stat.reload();
    assert!(stat.irq_pending());
    GP1.ack_irq();
    stat.reload();
    assert!(!stat.irq_pending());
}

#[test_case]
fn display() {
    let mut stat = GPUSTAT::load();
    assert!(!stat.display_enabled());
    GP1.enable_display(true);
    stat.reload();
    assert!(stat.display_enabled());
    GP1.enable_display(false);
    stat.reload();
    assert!(!stat.display_enabled());
}

#[test_case]
fn dma_mode() {
    GP1.dma_mode(None);
    let mut stat = GPUSTAT::load();
    assert!(!stat.dma_enabled());
    GP1.dma_mode(Some(DMAMode::GP0));
    stat.reload();
    assert!(stat.dma_enabled());
    GP1.dma_mode(Some(DMAMode::GPUREAD));
    stat.reload();
    assert!(stat.dma_enabled());
}
