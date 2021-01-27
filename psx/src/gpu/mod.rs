use crate::hal::GP1;

mod color;
mod disp_env;
mod draw_env;
mod texture;
mod vertex;

pub use disp_env::DispEnv;
pub use draw_env::DrawEnv;

pub fn reset_graphics(res: (i16, i16), mode: VideoMode, depth: Depth, interlace: bool) {
    GP1.display_mode(Vertex::new(res), mode, depth, interlace)
        .enable_display(true);
}

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
    pub x: i16,
    pub y: i16,
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

pub type Command = u8;

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
