//! Double-buffered framebuffer routines

use crate::dma;
use crate::gpu::{reset_graphics, Color, Coordinate, Depth, DispEnv, DrawEnv, Vertex, VideoMode};
use crate::gpu::{GREEN, INDIGO};

/// Configuration for a double-buffered framebuffer.
pub struct Framebuffer {
    disp_envs: (DispEnv, DispEnv),
    draw_envs: (DrawEnv, DrawEnv),
    swapped: bool,
}

impl Framebuffer {
    /// Creates a framebuffer with buffers with resolution `res` starting at
    /// `buffer_0` and `buffer_1` and background color `bg_color`. If `bg_color`
    /// is `None`, buffers are not cleared after each swap.
    pub const fn uninit(
        buffer_0: Coordinate, buffer_1: Coordinate, res: Coordinate, bg_color: Option<Color>,
    ) -> Self {
        let buffer_0 = Vertex::new(buffer_0);
        let buffer_1 = Vertex::new(buffer_1);
        let res = Vertex::new(res);
        let disp_envs = (DispEnv::new(buffer_0, res), DispEnv::new(buffer_1, res));
        let draw_envs = (
            DrawEnv::new(buffer_1, res, Some(GREEN)),  //bg_color),
            DrawEnv::new(buffer_0, res, Some(INDIGO)), //bg_color),
        );
        Framebuffer {
            disp_envs,
            draw_envs,
            swapped: false,
        }
    }

    pub fn new(
        buffer_0: Coordinate, buffer_1: Coordinate, res: Coordinate, bg_color: Option<Color>,
        mode: VideoMode, depth: Depth, interlace: bool,
    ) -> Self {
        let mut fb = Framebuffer::uninit(buffer_0, buffer_1, res, bg_color);
        fb.init(mode, depth, interlace);
        fb
    }

    pub fn init(&mut self, mode: VideoMode, depth: Depth, interlace: bool) -> &mut Self {
        reset_graphics(self.draw_envs.0.resolution(), mode, depth, interlace);
        self.swap(None);
        self
    }

    /// Swaps the currently displayed buffer.
    pub fn swap(&mut self, gpu_dma: Option<&mut dma::GPU>) {
        self.swapped = !self.swapped;
        let (disp_env, draw_env) = self.envs();
        disp_env.set();
        match gpu_dma {
            Some(gpu_dma) => gpu_dma.send_list(draw_env),
            None => draw_env.set(),
        }
    }

    fn envs(&self) -> (&DispEnv, &DrawEnv) {
        if self.swapped {
            (&self.disp_envs.1, &self.draw_envs.1)
        } else {
            (&self.disp_envs.0, &self.draw_envs.0)
        }
    }
}
