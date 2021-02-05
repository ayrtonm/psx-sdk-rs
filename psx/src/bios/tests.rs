#![cfg(test)]
use crate::bios;
use crate::gpu::DMAMode;
use crate::hal::{Register, GP1, GPUSTAT};

#[test_case]
fn save_state() {
    let mut buf = [0u32; 16];
    // Stores registers in first 12 spots
    bios::save_state(&mut buf);
    assert!(buf[0] != 0); // contains $ra
    assert!(buf[4] != 0); // contains $sp
                          // Remaining registers may or may not be zero
    assert!(buf[12] == 0); // Extra space in buffer should remain zeroed
    assert!(buf[13] == 0);
    assert!(buf[14] == 0);
    assert!(buf[15] == 0);
}

#[test_case]
fn rand() {
    const N: usize = 8 * 1024;
    bios::srand(0xC0FFEE);
    for _ in 0..N {
        assert!((bios::rand() >> 15) == 0);
    }
}

#[test_case]
fn srand() {
    let seed = 0xDEAD_BEEF;
    bios::srand(seed);
    let expected_value = ((((seed * 0x41C6_4E6D) + 12345) / 0x1_0000) & 0x7FFF) as u16;
    assert!(bios::rand() == expected_value);
}

#[test_case]
fn gp1_command() {
    GP1.reset_gpu().enable_display(true);
    let old_status = GPUSTAT::load().bits();
    bios::gp1_command(0);
    let new_status = GPUSTAT::load().bits();
    assert!(old_status == 0x14002000);
    assert!(new_status == 0x14802000);
}

#[test_case]
fn gp0_command() {
    let arg = 0xC0FFEE;
    let lower_bits = (1 << 11) - 1;
    bios::gp0_command(0xe1 << 24 | arg);
    let status = GPUSTAT::load().bits();
    assert!(status & lower_bits == arg & lower_bits);
}

#[test_case]
fn gp0_command_params() {
    let arg = 0xBEEF;
    let lower_bits = (1 << 11) - 1;
    let cmd = [0, 0, 0xe1 << 24 | arg, 0, 0];
    bios::gp0_command_params(&cmd);
    let status = GPUSTAT::load().bits();
    assert!(status & lower_bits == arg & lower_bits);
}

#[test_case]
fn gpu_get_status() {
    let reg_status = GPUSTAT::load();
    let bios_status = bios::gpu_get_status();
    assert!(reg_status == bios_status);
}

#[test_case]
fn gpu_sync() {
    GP1.dma_mode(Some(DMAMode::GP0));
    assert!(GPUSTAT::load().dma_enabled());
    let timeout = bios::gpu_sync();
    assert!(!GPUSTAT::load().dma_enabled());
    assert!(!timeout);
}

#[test_case]
fn system_date() {
    // This is for SCPH-5500
    assert!(bios::system_date() == 0x19951204);
}

#[test_case]
fn system_version() {
    assert!(bios::system_version() == "CEX-3000/1001/1002 by K.S.");
}

#[test_case]
fn system_ram() {
    assert!(bios::system_ram() == 2 * 1024);
}

#[test_case]
fn enter_critical_section() {
    // Ensure we're outside a critical section
    bios::exit_critical_section();
    let first_entry = bios::enter_critical_section();
    let reentry = bios::enter_critical_section();
    assert!(first_entry);
    assert!(!reentry);
}

#[test_case]
fn critical_section() {
    // Ensure we're outside a critical section
    bios::exit_critical_section();
    let res = bios::critical_section(|| {
        assert!(!bios::enter_critical_section());
        0xdeadbeefu32
    });
    assert!(res == 0xdeadbeef);
}
