use psx::dma;
use psx::gpu::{DMAMode, Depth, DispEnv, DrawEnv, NTSC, Packet};
use psx::hw::gpu::{GP0, GP1};
use psx::Result;

pub struct Framebuffer {
    pub gp0: GP0,
    gp1: GP1,
    disp_envs: [DispEnv; 2],
    draw_envs: [Packet<DrawEnv>; 2],
    swapped: bool,
}

impl Framebuffer {
    pub fn new(buf0: (i16, i16), buf1: (i16, i16), res: (i16, i16)) -> Result<Self> {
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
            Some(dma) => {
                //dma.send_list_and(&self.draw_envs[idx], || ())?;
                self.gp0.send_command(&self.draw_envs[idx].payload);
            }
            None => {
                self.gp0.send_command(&self.draw_envs[idx].payload);
            }
        }
        Ok(())
    }
}
