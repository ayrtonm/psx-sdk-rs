use crate::gpu::{PackedVertex, Vertex};
use crate::hal::GP1;

pub struct DispEnv {
    horizontal_range: PackedVertex<3, 12, 12>,
    vertical_range: PackedVertex<3, 10, 10>,
    offset: PackedVertex<3, 10, 9>,
}

impl DispEnv {
    pub const fn new(offset: Vertex, size: Vertex) -> Self {
        let ntsc_vert_range = Vertex::new((0x88 - (224 / 2), 0x88 + (224 / 2)));
        let horiz_range = Vertex::new((0x260, 0x260 + (size.x * 8)));
        DispEnv {
            offset: PackedVertex::new(offset),
            horizontal_range: PackedVertex::new(horiz_range),
            vertical_range: PackedVertex::new(ntsc_vert_range),
        }
    }

    pub fn set(&self) {
        GP1.display_start(self.offset)
            .horizontal_range(self.horizontal_range)
            .vertical_range(self.vertical_range);
    }
}
