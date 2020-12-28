use crate::gpu::{PackedVertex, Vertex};

/// Display environment settings.
pub struct DispEnv {
    /// The horizontal range of the display area.
    pub horizontal_range: PackedVertex<12, 12>,
    /// The vertical range of the display area.
    pub vertical_range: PackedVertex<10, 10>,
    /// The start of the display area in VRAM.
    pub offset: PackedVertex<10, 9>,
    //pub disp_mode: u8,
}

impl DispEnv {
    /// Constructs a new display environment.
    pub fn new<T: Copy, U: Copy>(offset: T, size: U) -> Self
    where Vertex: From<T> + From<U> {
        let size = Vertex::from(size);
        DispEnv {
            offset: offset.into(),
            horizontal_range: Vertex::from((0, size.x * 8)).shift(0x260).into(),
            vertical_range: Vertex::from((-1, 1)).scale(size.y / 2).shift(0x88).into(),
            //disp_mode: 0x01,
        }
    }
}
