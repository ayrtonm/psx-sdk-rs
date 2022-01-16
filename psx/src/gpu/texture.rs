use crate::gpu::{Clut, PackedVertex, TexPage, Vertex, VertexError};

impl TexPage {
    pub fn try_from(v: Vertex) -> Result<Self, VertexError> {
        Ok(Self(PackedVertex::try_from(v)?))
    }
}

impl Clut {
    pub fn try_from(v: Vertex) -> Result<Self, VertexError> {
        Ok(Self(PackedVertex::try_from(v)?))
    }
}
