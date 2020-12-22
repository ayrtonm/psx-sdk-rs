use super::vertex::PackedVertex;

pub struct DrawEnv {
    tex_page_cmd: u8,
    tex_page: [u8; 3],
    tex_win_cmd: u8,
    tex_win: [u8; 3],
    top_left_cmd: u8,
    top_left: PackedVertex<10, 9>,
    bot_right_cmd: u8,
    bot_right: PackedVertex<10, 9>,
    offset_cmd: u8,
    offset: PackedVertex<11, 11>,
}

impl DrawEnv {
    pub fn new<T, U, V>(top_left: T, bot_right: U, offset: V) -> Self
    where PackedVertex<10, 9>: From<T> + From<U>, PackedVertex<11, 11>: From<V> {
        DrawEnv {
            tex_page_cmd: 0xE1,
            tex_win_cmd: 0xE2,
            top_left_cmd: 0xE3,
            bot_right_cmd: 0xE4,
            offset_cmd: 0xE5,

            tex_page: [0; 3],
            tex_win: [0; 3],
            top_left: top_left.into(),
            bot_right: bot_right.into(),
            offset: offset.into(),
        }
    }
}
