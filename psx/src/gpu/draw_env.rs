use super::vertex::PackedVertex;
use crate::graphics::{Packet, Primitive};

#[repr(C)]
pub struct DrawEnv {
    //tex_page_cmd: u8,
    //tex_page: [u8; 3],
    //tex_win_cmd: u8,
    //tex_win: [u8; 3],
    upper_left_cmd: u8,
    upper_left: PackedVertex<10, 9>,
    lower_right_cmd: u8,
    lower_right: PackedVertex<10, 9>,
    offset_cmd: u8,
    offset: PackedVertex<11, 11>,
}

impl DrawEnv {
    pub fn new<T, U, V>(upper_left: T, lower_right: U, offset: V) -> Packet<Self>
    where
        PackedVertex<10, 9>: From<T> + From<U>,
        PackedVertex<11, 11>: From<V>, {
        Packet::new(DrawEnv {
            //tex_page_cmd: 0xE1,
            //tex_win_cmd: 0xE2,
            upper_left_cmd: 0xE3,
            lower_right_cmd: 0xE4,
            offset_cmd: 0xE5,

            //tex_page: [0; 3],
            //tex_win: [0; 3],
            upper_left: upper_left.into(),
            lower_right: lower_right.into(),
            offset: offset.into(),
        })
    }
}

impl Primitive for DrawEnv {
    fn init(&mut self) {}
}
