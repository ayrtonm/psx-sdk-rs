//! Basic GPU data types

use crate::hal::{MutRegister, Mutable, Register, GP1, GPUSTAT, I_STAT};
use crate::interrupt::IRQ;

pub use crate::hal::gpu::ty::{Bpp, Clut, Color, Command, Coordinate, DMAMode, Depth, PackedVertex,
                              Pixel, TexCoord, TexPage, Vertex, VideoMode};
pub use crate::hal::gpu::Primitive;

pub fn reset_graphics<T: Into<Vertex>>(res: T, mode: VideoMode, depth: Depth, interlace: bool) {
    GP1.reset_gpu()
        .dma_mode(Some(DMAMode::GP0))
        .display_mode(res.into(), mode, depth, interlace)
        .enable_display(true);
}

pub fn draw_sync() {
    let mut gpu_stat = GPUSTAT::load();
    if gpu_stat.dma_enabled() {
        while !(gpu_stat.cmd_ready() && gpu_stat.dma_ready()) {
            gpu_stat.reload();
        }
    } else {
        gpu_stat.wait_cmd();
    }
}

pub fn vsync() {
    I_STAT::<Mutable>::load()
        .ack(IRQ::Vblank)
        .store()
        .wait(IRQ::Vblank);
}

pub const BLACK: Color = Color::new(0, 0, 0);
pub const WHITE: Color = Color::new(0xFF, 0xFF, 0xFF);
pub const RED: Color = Color::new(0xFF, 0, 0);
pub const GREEN: Color = Color::new(0, 0xFF, 0);
pub const BLUE: Color = Color::new(0, 0, 0xFF);

pub const YELLOW: Color = RED.average(GREEN);
pub const CYAN: Color = GREEN.average(BLUE);
pub const VIOLET: Color = BLUE.average(RED);

pub const PINK: Color = RED.average(VIOLET);
pub const ORANGE: Color = RED.average(YELLOW);
pub const LIME: Color = GREEN.average(YELLOW);
pub const MINT: Color = GREEN.average(CYAN);
pub const AQUA: Color = BLUE.average(CYAN);
pub const INDIGO: Color = BLUE.average(VIOLET);

pub const NTSC: VideoMode = VideoMode::NTSC;
pub const PAL: VideoMode = VideoMode::PAL;
