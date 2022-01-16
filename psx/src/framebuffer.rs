use crate::dma;
use crate::gpu::{Color, DMAMode, Depth, DispEnv, DrawEnv, Packet, Vertex, VertexError,
                 VertexdeoMode};
use crate::hw::gpu;
use crate::hw::gpu::{GP0, GP1};
use crate::hw::irq;
use crate::hw::Register;
use crate::IRQ;

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
    pub fn new(
        buf0: Vertex, buf1: Vertex, res: Vertex, bg_color: Option<Color>,
    ) -> Result<Self, VertexError> {
        let mut fb = Framebuffer {
            gp0: GP0::new(),
            gp1: GP1::new(),
            disp_envs: [DispEnv::new(buf0, res)?, DispEnv::new(buf1, res)?],
            draw_envs: [
                Packet::new(DrawEnv::new(buf1, res, bg_color)?),
                Packet::new(DrawEnv::new(buf0, res, bg_color)?),
            ],
            swapped: false,
        };
        GP1::new()
            .reset_gpu()
            .dma_mode(Some(DMAMode::GP0))
            .display_mode(res, VertexdeoMode::NTSC, Depth::High, false)?
            .enable_display(true);
        fb.swap(None);
        Ok(fb)
    }

    pub fn swap(&mut self, gpu_dma: Option<&mut dma::GPU>) {
        self.swapped = !self.swapped;
        let idx = self.swapped as usize;
        self.gp1.set_display_env(&self.disp_envs[idx]);
        match gpu_dma {
            Some(dma) => dma.send_list(&self.draw_envs[idx]),
            None => {
                self.gp0.send_command(&self.draw_envs[idx].contents);
            },
        }
    }
}
