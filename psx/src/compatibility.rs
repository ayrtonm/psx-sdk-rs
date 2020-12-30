#![allow(non_snake_case)]
use core::hint::unreachable_unchecked;

use crate::bios;
use crate::value::{Load, LoadMut};

use crate::dma;
use crate::dma::{BlockControl, BlockMode, Channel};
use crate::gpu::{DispEnv, DrawEnv};
use crate::graphics::packet::Packet;
use crate::graphics::LinkedList;
use crate::timer::{Source, SyncMode, TimerCounter};

use crate::dma::{DICR, DPCR};
use crate::gpu::{GP1, GPUSTAT};
use crate::irq::IMASK;
use crate::timer::timer1;

static mut VSYNC_LASTHBLANK: u16 = 0;
static mut VSYNC_RCNT: u32 = 0;

/// Executes the given closure in a critical section and returns the result.
#[inline(always)]
pub fn CriticalSection<F: FnOnce() -> R, R>(f: F) -> R {
    bios::enter_critical_section();
    let r = f();
    bios::exit_critical_section();
    r
}

/// Resets the GPU and installs a VSync event handler
#[no_mangle]
pub fn ResetGraph(mode: u32, gpu_dma: &mut dma::gpu::CHCR) {
    CriticalSection(|| {
        DPCR.skip_load()
            .disable_all()
            .enable(Channel::GPU)
            .enable(Channel::OTC)
            .store();
        DICR.skip_load().clear_all().store();
        IMASK.skip_load().clear_all().store();
        bios::cd_remove();
    });
    timer1::MODE
        .skip_load()
        .sync_mode(SyncMode::Pause)
        .source(Source::Alternate)
        .store();
    match mode {
        1 => {
            gpu_dma.skip_load().stop().store();
            GP1.reset_command_buffer();
        },
        3 => {
            GP1.reset_command_buffer();
        },
        _ => {
            GP1.reset_gpu();
        },
    }
}

/// Waits for drawing to terminate if `mode == 0`. Otherwise returns the number
/// of positions in the current queue.
#[no_mangle]
pub fn DrawSync(mode: u32, gpu_dma: &dma::gpu::CHCR) -> u16 {
    if mode == 0 {
        if GPUSTAT.load().dma_enabled() {
            while gpu_dma.load().busy() {}
            while {
                let stat = GPUSTAT.load();
                !(stat.cmd_ready() && stat.dma_ready())
            } {}
            5
        } else {
            while !GPUSTAT.load().dma_ready() {}
            1
        }
    } else {
        if let Some(BlockMode::Multi { words: _, blocks }) =
            dma::gpu::BCR.get(dma::TransferMode::Request)
        {
            blocks
        } else {
            unsafe { unreachable_unchecked() }
        }
    }
}

/// Waits for the next vertical blank or return the vertical blank counter
/// value.
#[no_mangle]
pub fn VSync(mode: i32) -> u32 {
    let mut stat = GPUSTAT.load();
    let hblank = timer1::CNT.wait();
    let ret = unsafe { (hblank - VSYNC_LASTHBLANK) as u32 };
    match mode {
        i32::MIN..0 => unsafe { VSYNC_RCNT },
        1 => unsafe {
            VSYNC_LASTHBLANK = timer1::CNT.wait();
            ret
        },
        _ => {
            let vblanks = if mode == 0 { 1 } else { mode as u32 };
            unsafe {
                fn vsync_sub(vsync_tgt: u32, _a1: u32) {
                    if unsafe { vsync_tgt > VSYNC_RCNT } {
                        bios::change_clear_pad(0);
                        bios::change_clear_rcnt(3, 0);
                    }
                }
                vsync_sub(VSYNC_RCNT + vblanks, vblanks + 1);
                if stat.interlaced() {
                    let mut new_stat = GPUSTAT.load();
                    while stat.line() == new_stat.line() {
                        stat = new_stat;
                        new_stat = GPUSTAT.load();
                    }
                } else {
                    VSYNC_LASTHBLANK = timer1::CNT.wait();
                }
                ret
            }
        },
    }
}

/// Sets the display mask.
#[no_mangle]
pub fn SetDispMask(mut mode: u32) {
    mode &= 1;
    GP1.display_enable(mode != 0);
}

/// Sends an ordering table through to the GPU via DMA.
pub fn DrawOTag<L: LinkedList>(list: &L, gpu_dma: &mut dma::gpu::CHCR) {
    let _ = gpu_dma.send_list(list);
}

/// Sets the display environment.
#[no_mangle]
pub fn PutDispEnv(disp_env: &DispEnv) {
    disp_env.set();
}

/// Sets the drawing environment.
#[no_mangle]
pub fn PutDrawEnv(draw_env: &Packet<DrawEnv>, gpu_dma: &mut dma::gpu::CHCR) {
    let _ = gpu_dma.send_list(draw_env);
}
