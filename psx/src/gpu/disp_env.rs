use crate::gpu::{PackedVertex, Vertex, GP1};

/// Display environment settings.
pub struct DispEnv {
    horizontal_range: PackedVertex<3, 12, 12>,
    vertical_range: PackedVertex<3, 10, 10>,
    offset: PackedVertex<3, 10, 9>,
    //video_mode: Option<VideoMode>,
}

impl DispEnv {
    /// Constructs a new display environment.
    pub fn new<T: Copy, U: Copy>(
        offset: T, size: U, /* , video_mode: Option<VideoMode> */
    ) -> Self
    where Vertex: From<T> + From<U> {
        let size = Vertex::from(size);
        DispEnv {
            offset: offset.into(),
            horizontal_range: Vertex::from((0, size.x * 8)).shift(0x260).into(),
            vertical_range: Vertex::from((-1, 1)).scale(224 / 2).shift(0x88).into(),
            //disp_mode: 0x01,
        }
    }

    /// Sets the display environment.
    #[inline(always)]
    pub fn set(&self) {
        GP1.start_display_area(self.offset)
            .horizontal_range(self.horizontal_range)
            .vertical_range(self.vertical_range);
    }
}
