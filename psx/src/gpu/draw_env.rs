use crate::gpu::{Color, Command, PackedVertex, Vertex, BLACK};
use crate::graphics::{AsSlice, Packet};
use crate::hal::GP0;

#[repr(C)]
#[derive(Debug)]
pub struct DrawEnv(Packet<InnerDrawEnv>);

#[repr(C)]
#[derive(Debug)]
struct InnerDrawEnv {
    texpage: u16,
    _pad: u8,
    texpage_cmd: Command,

    upper_left: PackedVertex<3, 10, 9>,
    upper_left_cmd: Command,

    lower_right: PackedVertex<3, 10, 9>,
    lower_right_cmd: Command,

    offset: PackedVertex<3, 11, 11>,
    offset_cmd: Command,

    bg_color: Color,
    bg_color_cmd: Command,
    bg_offset: Vertex,
    bg_size: Vertex,
}

impl AsSlice for InnerDrawEnv {}

impl DrawEnv {
    pub const fn new(offset: Vertex, size: Vertex, bg_color: Option<Color>) -> Self {
        let bg_color = match bg_color {
            Some(color) => color,
            None => BLACK,
        };
        let draw_env = InnerDrawEnv {
            texpage_cmd: 0xE1,
            upper_left_cmd: 0xE3,
            lower_right_cmd: 0xE4,
            offset_cmd: 0xE5,
            bg_color_cmd: 0x02,

            texpage: (1 << 10) | 10,

            upper_left: PackedVertex::new(offset),

            lower_right: PackedVertex::new(offset.shift(size)),

            offset: PackedVertex::new(offset),

            bg_color,
            bg_offset: offset,
            bg_size: size,

            _pad: 0,
        };
        DrawEnv(Packet::new(draw_env, None))
    }

    pub fn resolution(&self) -> Vertex {
        self.0.bg_size
    }

    pub fn set(&self) {
        GP0.write_slice(self.0.as_slice());
    }
}
