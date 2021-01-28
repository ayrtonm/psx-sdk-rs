//! Double-buffered framebuffer routines

use crate::gpu::{Color, Vertex};
use crate::gpu::{DispEnv, DrawEnv};

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
    pub const fn new(
        buffer_0: (i16, i16), buffer_1: (i16, i16), res: (i16, i16), bg_color: Option<Color>,
    ) -> Self {
        let buffer_0 = Vertex::new(buffer_0);
        let buffer_1 = Vertex::new(buffer_1);
        let res = Vertex::new(res);
        let disp_envs = (DispEnv::new(buffer_0, res), DispEnv::new(buffer_1, res));
        let draw_envs = (
            DrawEnv::new(buffer_1, res, bg_color),
            DrawEnv::new(buffer_0, res, bg_color),
        );
        Framebuffer {
            disp_envs,
            draw_envs,
            swapped: false,
        }
    }

    /// Swaps the currently displayed buffer.
    pub fn swap(&mut self) {
        self.swapped = !self.swapped;
        let (disp_env, draw_env) = self.envs();
        disp_env.set();
        draw_env.set();
    }

    fn envs(&self) -> (&DispEnv, &DrawEnv) {
        if self.swapped {
            (&self.disp_envs.0, &self.draw_envs.0)
        } else {
            (&self.disp_envs.1, &self.draw_envs.1)
        }
    }
}
