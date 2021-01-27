use crate::gpu::{Clut, TexCoord};

impl From<Option<Clut>> for Clut {
    fn from(clut: Option<Clut>) -> Self {
        clut.unwrap_or((0, 0).into())
    }
}

impl From<(u8, u8)> for TexCoord {
    fn from((x, y): (u8, u8)) -> TexCoord {
        TexCoord { x, y }
    }
}
