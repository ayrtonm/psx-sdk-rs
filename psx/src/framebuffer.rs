use crate::dma;
use crate::gpu::{Color, DispEnv, DrawEnv, Vertex};
use crate::graphics::packet::Packet;

/// Framebuffer containing display and draw environment settings.
pub struct Framebuffer {
    disp_envs: (DispEnv, DispEnv),
    draw_envs: (Packet<DrawEnv>, Packet<DrawEnv>),
    swapped: bool,
}

impl Framebuffer {
    /// Creates a new framebuffer.
    pub fn new<T: Copy, U: Copy, V: Copy>(
        buffer_0: T, buffer_1: U, res: V, bg_color: Option<Color>, gpu_dma: &mut dma::gpu::CHCR,
    ) -> Self
    where Vertex: From<T> + From<U> + From<V> {
        let bg_color = bg_color.unwrap_or(Color::BLACK);
        let disp_envs = (DispEnv::new(buffer_0, res), DispEnv::new(buffer_1, res));
        let draw_envs = (
            DrawEnv::new(buffer_1, res, bg_color),
            DrawEnv::new(buffer_0, res, bg_color),
        );
        disp_envs.0.set();
        gpu_dma.send_list(&draw_envs.1).wait();
        Framebuffer {
            disp_envs,
            draw_envs,
            swapped: false,
        }
    }

    /// Swaps the draw and display buffers.
    pub fn swap(&mut self, gpu_dma: &mut dma::gpu::CHCR) {
        self.swapped = !self.swapped;
        let (disp_env, draw_env) = if self.swapped {
            (&self.disp_envs.1, &self.draw_envs.0)
        } else {
            (&self.disp_envs.0, &self.draw_envs.1)
        };
        disp_env.set();
        gpu_dma.send_list(draw_env).wait();
    }
}
