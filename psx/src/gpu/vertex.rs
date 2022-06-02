use crate::gpu::{PackedVertex, Vertex, VertexError};
use core::convert::TryFrom;
use core::ops::{Add, AddAssign, Sub, SubAssign};

impl Add for Vertex {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for Vertex {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Vertex {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl SubAssign for Vertex {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl From<Vertex> for u32 {
    fn from(Vertex(x, y): Vertex) -> u32 {
        (x as u32) | ((y as u32) << 16)
    }
}

impl Vertex {
    /// Creats a new `Vertex`.
    pub const fn new((x, y): (i16, i16)) -> Self {
        Vertex(x, y)
    }
}

impl<const N: usize, const X: usize, const Y: usize> PackedVertex<N, X, Y> {
    const VALIDATE_X_PLUS_Y: () = {
        if X + Y > (N * 8) {
            panic!("Vertex elements are larger than backing array");
        }
    };
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

impl<const N: usize, const X: usize, const Y: usize> TryFrom<Vertex> for PackedVertex<N, X, Y> {
    type Error = VertexError;

    #[allow(path_statements)]
    fn try_from(Vertex(x, y): Vertex) -> Result<Self, VertexError> {
        // This is a compile-time check ensuring X + Y bits =< N bytes
        Self::VALIDATE_X_PLUS_Y;
        let x = x as u16;
        let y = y as u16;
        if x >= 1 << X {
            return Err(VertexError::InvalidX)
        }
        if y >= 1 << Y {
            return Err(VertexError::InvalidY)
        }
        let mut data = [0; N];
        let value = (x as u32 | ((y as u32) << X)).to_le_bytes();
        for i in 0..data.len() {
            data[i] = value[i];
        }
        Ok(PackedVertex { data })
    }
}

impl<const N: usize, const X: usize, const Y: usize> From<PackedVertex<N, X, Y>> for Vertex {
    fn from(packed: PackedVertex<N, X, Y>) -> Vertex {
        let mut ar = [0; 4];
        for i in 0..packed.data.len() {
            ar[i] = packed.data[i];
        }
        let data = u32::from_le_bytes(ar);
        let x_mask = (1 << X) - 1;
        let y_mask = (1 << Y) - 1;
        let y_shift = X;
        let x = data & x_mask;
        let y = (data >> y_shift) & y_mask;
        Vertex(x as i16, y as i16)
    }
}

#[cfg(test)]
mod tests {

    use super::PackedVertex;
    use crate::gpu::{Vertex, VertexError};

    macro_rules! with_bytes {
        ($bytes:expr, { $($body:tt)* }) => {
            {
                const N: usize = $bytes;
                const BITS: usize = $bytes * 8;
                $($body)*
            }
        };
    }

    macro_rules! create_packed {
        () => {
            fuzz!(|x: i16, y: i16| {
                const X: usize = const_random!(usize) % 16;
                const UNUSED_BITS: usize = const_random!(usize) % (BITS - X);
                const Y: usize = (BITS - X - UNUSED_BITS) % 16;
                let packed = PackedVertex::<N, X, Y>::try_from(Vertex(x, y));
                let x_too_big = x as u32 >= 1 << X;
                let y_too_big = y as u32 >= 1 << Y;
                if x_too_big {
                    assert!(packed == Err(VertexError::InvalidX));
                } else if y_too_big {
                    assert!(packed == Err(VertexError::InvalidY));
                } else {
                    assert!(packed.is_ok());
                }
            });
        };
    }

    #[test_case]
    fn create_packed() {
        with_bytes!(2, {
            create_packed!();
        });
        with_bytes!(3, {
            create_packed!();
        });
        with_bytes!(4, {
            create_packed!();
        });
    }

    macro_rules! round_trip {
        () => {
            fuzz!(|x: u16, y: u16| {
                const X: usize = const_random!(usize) % 16;
                // This is only a lower limit on the number of unused bits
                const UNUSED_BITS: usize = const_random!(usize) % (BITS - X);
                const Y: usize = (BITS - X - UNUSED_BITS) % 16;
                let valid_x = (x % (1 << X)) as i16;
                let valid_y = (y % (1 << Y)) as i16;
                let packed = PackedVertex::<N, X, Y>::try_from(Vertex(valid_x, valid_y));
                assert!(packed.is_ok());
                let Vertex(new_x, new_y) = packed.unwrap().into();
                assert!(new_x == valid_x);
                assert!(new_y == valid_y);
            });
        };
    }

    #[test_case]
    fn round_trip() {
        with_bytes!(2, {
            round_trip!();
        });
        with_bytes!(3, {
            round_trip!();
        });
        with_bytes!(4, {
            round_trip!();
        });
    }
}
