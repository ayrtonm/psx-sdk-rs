use crate::gpu::{PackedVertex, Vertex};

impl From<Vertex> for u32 {
    fn from(vertex: Vertex) -> u32 {
        vertex.x as u32 | (vertex.y as u32) << 16
    }
}

impl From<(i16, i16)> for Vertex {
    fn from((x, y): (i16, i16)) -> Vertex {
        Vertex { x, y }
    }
}

impl Vertex {
    pub const fn new((x, y): (i16, i16)) -> Vertex {
        Vertex { x, y }
    }

    pub const fn shift(&self, other: Vertex) -> Vertex {
        Vertex {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub const fn scale(&self, scale: Vertex) -> Vertex {
        Vertex {
            x: self.x * scale.x,
            y: self.y * scale.y,
        }
    }
}

impl<const X: usize, const Y: usize> From<PackedVertex<2, X, Y>> for u32 {
    fn from(vertex: PackedVertex<2, X, Y>) -> u32 {
        vertex.data[0] as u32 | (vertex.data[1] as u32) << 8
    }
}

impl<const X: usize, const Y: usize> From<PackedVertex<3, X, Y>> for u32 {
    fn from(vertex: PackedVertex<3, X, Y>) -> u32 {
        vertex.data[0] as u32 | (vertex.data[1] as u32) << 8 | (vertex.data[2] as u32) << 16
    }
}

impl<T, const N: usize, const X: usize, const Y: usize> From<T> for PackedVertex<N, X, Y>
where Vertex: From<T>
{
    fn from(t: T) -> Self {
        Self::new(t.into())
    }
}

impl<const N: usize, const X: usize, const Y: usize> PackedVertex<N, X, Y> {
    pub const fn new(v: Vertex) -> PackedVertex<N, X, Y> {
        let mut data = [0; N];
        let value = ((v.x as u32) | ((v.y as u32) << X)).to_le_bytes();
        let mut i = 0;
        while i < N {
            data[i] = value[i];
            i += 1;
        }
        PackedVertex { data }
    }
}
