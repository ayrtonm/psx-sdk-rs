//! Basic GPU data types

use crate::hal::{MutRegister, Register, GP1, GPUSTAT, I_STAT};
use crate::interrupt::IRQ;

mod color;
mod disp_env;
mod draw_env;
mod texture;
mod vertex;

pub use disp_env::DispEnv;
pub use draw_env::DrawEnv;

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
    I_STAT::load_mut()
        .ack(IRQ::Vblank)
        .store()
        .wait(IRQ::Vblank);
}

pub type Pixel = i16;
pub type Command = u8;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Vertex {
    pub x: Pixel,
    pub y: Pixel,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PackedVertex<const N: usize, const X: usize, const Y: usize> {
    data: [u8; N],
}

pub type Clut = PackedVertex<2, 6, 9>;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TexCoord {
    pub x: u8,
    pub y: u8,
}

pub type TexPage = PackedVertex<2, 4, 1>;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DMAMode {
    GP0 = 2,
    GPUREAD,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VideoMode {
    NTSC = 0,
    PAL,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Depth {
    /// 15-bit high-color mode
    High = 0,
    /// 24-bit true-color mode
    True,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Bpp {
    Bit4,
    Bit8,
    Bit15,
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
