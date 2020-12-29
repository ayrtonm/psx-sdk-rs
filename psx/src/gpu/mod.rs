/// GP0 register.
pub mod gp0;
/// GP1 register.
pub mod gp1;
/// GPU Status register.
pub mod stat;

mod disp_env;
mod draw_env;

mod color;
mod texture;
mod vertex;

pub use color::Color;
pub use disp_env::DispEnv;
pub use draw_env::DrawEnv;
pub use gp0::GP0;
pub use gp1::GP1;
pub use stat::GPUSTAT;
pub use texture::{Clut, TexCoord, TexPage};
pub use vertex::{GenericVertex, PackedVertex, Pixel, SmallVertex, Vertex};

/// Parity of the interlaced line being drawn.
#[derive(PartialEq, Eq)]
pub enum Parity {
    /// Drawing an odd numbered line.
    Odd,
    /// Drawing an even numbered line or vblank.
    Even,
}

/// DMA Direction
pub enum Direction {
    /// GPU DMA disabled.
    Off = 0,
    /// TODO: ???
    FIFO,
    /// GPU DMA transfers from CPU to GP0.
    ToGPU,
    /// GPU DMA transfers from GPUREAD to CPU.
    ToCPU,
}

/// A video standard.
pub enum VideoMode {
    /// NTSC/60 Hz video.
    NTSC,
    /// PAL/50 Hz video.
    PAL,
}

/// Bits per pixel.
pub enum Bpp {
    /// 4-bits per pixel.
    Bit4,
    /// 8-bits per pixel.
    Bit8,
    /// 15-bits per pixel.
    Bit15,
}

#[inline(always)]
const fn cmd(cmd: u8) -> u32 {
    (cmd as u32) << 24
}
