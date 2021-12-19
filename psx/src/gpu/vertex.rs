use crate::gpu::{PackedVertex, Vertex};
use core::convert::TryFrom;

impl From<Vertex> for u32 {
    fn from(v: Vertex) -> u32 {
        v.x as u32 | (v.y as u32) << 16
    }
}

impl From<(i16, i16)> for Vertex {
    fn from((x, y): (i16, i16)) -> Vertex {
        Vertex { x, y }
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

impl<const N: usize, const X: usize, const Y: usize> TryFrom<(i16, i16)> for PackedVertex<N, X, Y> {
    type Error = &'static str;

    fn try_from(p: (i16, i16)) -> Result<Self, <Self as TryFrom<(i16, i16)>>::Error> {
        let v = Vertex::from(p);
        if v.x >= 1 << X {
            return Err("Could not encode x in PackedVertex")
        }
        if v.y >= 1 << Y {
            return Err("Could not encode y in PackedVertex")
        }
        let mut data = [0; N];
        let value = ((v.x as u32) | ((v.y as u32) << X)).to_le_bytes();
        let mut i = 0;
        while i < N {
            data[i] = value[i];
            i += 1;
        }
        Ok(PackedVertex { data })
    }
}
