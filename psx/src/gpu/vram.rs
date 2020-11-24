use crate::gpu::color::Color;
use crate::gpu::vertex::Vertex;
use crate::gpu::{AsU32, Packet};

struct VramSection {
    offset: Vertex,
    size: Vertex,
}

pub struct FillVram {
    src: VramSection,
    color: Color,
}

pub struct CopyVram {
    src: VramSection,
    dest: Vertex,
}

impl Packet<3> for FillVram {
    fn packet(&self) -> [u32; 3] {
        [
            0x02 << 24 | self.color.as_u32(),
            self.src.offset.as_u32(),
            self.src.size.as_u32(),
        ]
    }
}

impl Packet<4> for CopyVram {
    fn packet(&self) -> [u32; 4] {
        [
            0x80 << 24,
            self.src.offset.as_u32(),
            self.dest.as_u32(),
            self.src.size.as_u32(),
        ]
    }
}
