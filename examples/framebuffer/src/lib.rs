#![no_std]

use psx::dma;
use psx::gpu::{DMAMode, Depth, DispEnv, DrawEnv, Packet, NTSC};
use psx::hw::gpu::{GP0, GP1};
use psx::hw::{gpu, irq, Register};
use psx::irq::IRQ;

mod plane;
mod vector;
pub mod constants {
    pub use crate::plane::{XY, XY2, XZ, YZ};
    pub use crate::vector::{X, X2, Y, Y2, Z, ZERO, ZERO2};
}
pub use plane::{Plane2, Plane3};
pub use vector::{V2, V3};

pub type Result<T> = core::result::Result<T, &'static str>;

pub fn enable_vblank() {
    irq::Mask::new().enable_irq(IRQ::Vblank).store();
}

pub fn draw_sync() {
    let mut gpu_stat = gpu::Status::new();
    while !gpu_stat.cmd_ready() || !gpu_stat.dma_ready() {
        gpu_stat.load();
    }
}

pub fn vsync() {
    irq::Status::new()
        .ack(IRQ::Vblank)
        .store()
        .wait(IRQ::Vblank);
}

pub struct Framebuffer {
    pub gp0: GP0,
    gp1: GP1,
    disp_envs: [DispEnv; 2],
    draw_envs: [Packet<DrawEnv>; 2],
    swapped: bool,
}

impl Framebuffer {
    pub fn new(buf0: V2, buf1: V2, res: V2) -> Result<Self> {
        let buf0 = [buf0.0, buf0.1];
        let buf1 = [buf1.0, buf1.1];
        let res = [res.0, res.1];
        let mut fb = Framebuffer {
            gp0: GP0::new(),
            gp1: GP1::new(),
            disp_envs: [DispEnv::new(buf0, res)?, DispEnv::new(buf1, res)?],
            draw_envs: [
                Packet::new(DrawEnv::new(buf1, res, None)?)?,
                Packet::new(DrawEnv::new(buf0, res, None)?)?,
            ],
            swapped: false,
        };
        GP1::new()
            .reset_gpu()
            .dma_mode(Some(DMAMode::GP0))
            .display_mode(res, NTSC, Depth::High, false)?
            .enable_display(true);
        fb.swap(None)?;
        Ok(fb)
    }

    pub fn swap(&mut self, gpu_dma: Option<&mut dma::GPU>) -> Result<()> {
        self.swapped = !self.swapped;
        let idx = self.swapped as usize;
        self.gp1.set_display_env(&self.disp_envs[idx]);
        match gpu_dma {
            Some(dma) => dma.send_list(&self.draw_envs[idx])?,
            None => {
                self.gp0.send_command(&self.draw_envs[idx].payload);
            },
        }
        Ok(())
    }
}
