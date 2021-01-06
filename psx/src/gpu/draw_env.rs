use core::mem::size_of;

use crate::gpu::{Color, PackedVertex, Vertex};
use crate::graphics::packet::Packet;

/// Drawing environment settings.
#[repr(C)]
pub struct DrawEnv {
    texpage: u16,
    _pad: u8,
    texpage_cmd: u8,
    upper_left: PackedVertex<3, 10, 9>,
    upper_left_cmd: u8,
    lower_right: PackedVertex<3, 10, 9>,
    lower_right_cmd: u8,
    offset: PackedVertex<3, 11, 11>,
    offset_cmd: u8,
    // TODO: The following commands should be optional
    bg_color: Color,
    bg_color_cmd: u8,
    bg_offset: Vertex,
    bg_size: Vertex,
}

impl DrawEnv {
    /// Constructs a new drawing environment in a const context.
    pub const fn new_const(offset: Vertex, size: Vertex, bg_color: Option<Color>) -> Packet<Self> {
        // Subtract the 3 words for color command if no bg color was provided
        let (bg_color, packet_size) = match bg_color {
            Some(color) => (color, None),
            None => (Color::BLACK, Some((size_of::<Self>() / 4) as u32 - 3)),
        };
        Packet::new(
            DrawEnv {
                texpage_cmd: 0xE1,
                upper_left_cmd: 0xE3,
                lower_right_cmd: 0xE4,
                offset_cmd: 0xE5,
                bg_color_cmd: 0x02,

                // TODO: make this configurable
                texpage: (1 << 10) | 10,
                upper_left: PackedVertex::new(offset),
                lower_right: PackedVertex::new(Vertex::new(offset.x + size.x, offset.y + size.y)),
                offset: PackedVertex::new(offset),
                bg_color,
                bg_offset: offset,
                bg_size: size,

                _pad: 0,
            },
            packet_size,
        )
    }
    /// Constructs a new drawing environment.
    pub fn new<T: Copy, U: Copy>(offset: T, size: U, bg_color: Option<Color>) -> Packet<Self>
    where Vertex: From<T> + From<U> {
        let offset = Vertex::from(offset);
        let size = Vertex::from(size);
        // Subtract the 3 words for color command if no bg color was provided
        let packet_size = bg_color
            .is_none()
            .then_some((size_of::<Self>() / 4) as u32 - 3);
        Packet::new(
            DrawEnv {
                texpage_cmd: 0xE1,
                upper_left_cmd: 0xE3,
                lower_right_cmd: 0xE4,
                offset_cmd: 0xE5,
                bg_color_cmd: 0x02,

                // TODO: make this configurable
                texpage: (1 << 10) | 10,
                upper_left: offset.into(),
                lower_right: offset.shift(size).into(),
                offset: offset.into(),
                bg_color: bg_color.unwrap_or(Color::BLACK),
                bg_offset: offset.into(),
                bg_size: size.into(),

                _pad: 0,
            },
            packet_size,
        )
    }
}
