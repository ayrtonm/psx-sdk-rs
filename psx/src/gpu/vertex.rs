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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    OversizedX,
    OversizedY,
    InvalidX,
    InvalidY,
}

impl<const N: usize, const X: usize, const Y: usize> TryFrom<(i16, i16)> for PackedVertex<N, X, Y> {
    type Error = Error;

    fn try_from(p: (i16, i16)) -> Result<Self, Error> {
        let v = Vertex::from(p);
        if v.x >= 1 << X {
            return Err(Error::OversizedX)
        }
        if v.y >= 1 << Y {
            return Err(Error::OversizedY)
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

#[test_case]
fn create_packed() {
    fuzz!(|x: i16, y: i16| {
        const X: usize = const_random!(usize) % 16;
        const Y: usize = const_random!(usize) % 16;
        let packed = PackedVertex::<2, X, Y>::try_from((x, y));
        let x_too_big = x >= 1 << X;
        let y_too_big = y >= 1 << Y;
        if x_too_big {
            assert!(packed == Err(Error::OversizedX));
        } else if y_too_big {
            assert!(packed == Err(Error::OversizedY));
        } else {
            assert!(packed.is_ok());
        }
    });
}
