use crate::gpu::{PackedVertex, SmallVertex};

/// A color lookup table.
pub type Clut = PackedVertex<2, 6, 9>;

impl From<Option<Clut>> for Clut {
    fn from(clut: Option<Clut>) -> Self {
        clut.unwrap_or(0.into())
    }
}

/// A pair of coordinates within a texture page.
pub type TexCoord = SmallVertex;

// TODO: Add bpp to this
/// A pair of base texture page coordinates.
pub type TexPage = PackedVertex<2, 4, 1>;
