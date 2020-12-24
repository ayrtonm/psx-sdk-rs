#![allow(missing_docs)]
#![allow(warnings)]
use core::ffi::c_void;

use crate::bios;
use crate::cop0;
use crate::dma;
use crate::dma::{BlockControl, BlockMode, SyncMode};
use crate::gpu::stat::GPUStat;
use crate::value::{Load, LoadMut, Read};

/// Executes the given closure in a critical section and returns the result.
#[no_mangle]
pub fn critical_section<F: FnOnce() -> R, R>(f: F) -> R {
    bios::enter_critical_section();
    //cop0::Status.load_mut().enter_critical_section().store();
    let r = f();
    bios::exit_critical_section();
    //cop0::Status.load_mut().exit_critical_section().store();
    r
}

const reset_graph_msg: &'static [u8] = b"hello world\0";
const sr_msg: &'static [u8] = b"replace me\0";

/// Resets the GPU and installs a VSync event handler
#[no_mangle]
pub fn ResetGraph() {
    bios::printf(reset_graph_msg.as_ptr(), 0xdead_beef);
    bios::printf(sr_msg.as_ptr(), 0xdead_beef);
    critical_section(|| {
        dma::control::Control
            .skip_load()
            .disable_all()
            .enable(dma::Channel::GPU)
            .enable(dma::Channel::OTC)
            .store();
        dma::interrupt::Interrupt.skip_load().clear_all().store();
        InterruptCallback();
        RestartCallback();
        bios::cd_remove();
    });
}

/// Waits for drawing to terminate if `i == 0`. Otherwise returns the number of
/// positions in the current queue.
#[no_mangle]
pub fn DrawSync(i: u32) -> Option<u16> {
    use dma::gpu::{BCR, CHCR};
    if i == 0 {
        if GPUStat.load().dma_enabled() {
            // Wait for GPU to be ready for next DMA
            while CHCR.load().busy() {}
            while {
                let gpu_stat = GPUStat.load();
                !gpu_stat.cmd_ready() && !gpu_stat.dma_ready()
            } {}
        } else {
            while !GPUStat.load().dma_ready() {}
        }
        None
    } else {
        if let Some(BlockMode::Multi { words, blocks }) = BCR.get(SyncMode::Request) {
            Some(blocks)
        } else {
            unreachable!("")
        }
    }
}

#[no_mangle]
pub fn InterruptCallback() {}

#[no_mangle]
pub fn RestartCallback() {}

#[no_mangle]
pub fn VSync() {
    todo!("")
}
