#![cfg(test)]
use crate::gpu::DMAMode;
use crate::hw::gpu::{GP0, GP1};
use crate::hw::{gpu, Register};

fn stat_test(check: fn(&gpu::Status) -> bool, test: fn(&mut dyn FnMut() -> bool)) {
    let mut status = gpu::Status::new();
    let mut reload_and_check = || {
        status.load();
        check(&status)
    };
    test(&mut reload_and_check)
}

#[test_case]
fn reset_gpu() {
    GP1::new().reset_gpu();
    let status = gpu::Status::new().averaged_bits();
    assert!(status == 0x1480_2000);
}

#[test_case]
fn ack_irq() {
    stat_test(gpu::Status::irq_pending, |irq| {
        assert!(!irq());
        GP0::new().interrupt_request();
        assert!(irq());
        GP1::new().ack_irq();
        assert!(!irq());
    });
}

#[test_case]
fn display() {
    stat_test(gpu::Status::display_enabled, |disp| {
        let mut gp1 = GP1::new();

        assert!(!disp());
        gp1.enable_display(true);
        assert!(disp());
        gp1.enable_display(false);
        assert!(!disp());
    });
}

#[test_case]
fn dma_mode() {
    stat_test(gpu::Status::dma_enabled, |dma| {
        let mut gp1 = GP1::new();
        gp1.dma_mode(None);
        assert!(!dma());
        gp1.dma_mode(Some(DMAMode::GP0));
        assert!(dma());
        gp1.dma_mode(Some(DMAMode::GPUREAD));
        assert!(dma());
    });
}
