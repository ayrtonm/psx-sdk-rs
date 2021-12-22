use crate::gpu::{PackedVertex, Vertex};
use core::convert::TryFrom;
use strum_macros::IntoStaticStr;

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

#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoStaticStr)]
pub enum Error {
    OversizedX,
    OversizedY,
    InvalidX,
    InvalidY,
}

impl<const N: usize, const X: usize, const Y: usize> TryFrom<(i16, i16)> for PackedVertex<N, X, Y> {
    type Error = Error;

    fn try_from((x, y): (i16, i16)) -> Result<Self, Error> {
        let x = x as u16;
        let y = y as u16;
        if x >= 1 << X {
            return Err(Error::OversizedX)
        }
        if y >= 1 << Y {
            return Err(Error::OversizedY)
        }
        let mut data = [0; N];
        let value = (x as u32 | ((y as u32) << X)).to_le_bytes();
        for i in 0..data.len() {
            data[i] = value[i];
        }
        Ok(PackedVertex { data })
    }
}

impl<const N: usize, const X: usize, const Y: usize> PackedVertex<N, X, Y> {
    pub fn unpack(&self) -> (i16, i16) {
        let mut ar = [0; 4];
        for i in 0..self.data.len() {
            ar[i] = self.data[i];
        }
        let data = u32::from_le_bytes(ar);
        let x_mask = (1 << X) - 1;
        let y_mask = (1 << Y) - 1;
        let y_shift = X;
        let x = data & x_mask;
        let y = (data >> y_shift) & y_mask;
        (x as i16, y as i16)
    }
}

#[cfg(test)]
mod tests {

    use super::{Error, PackedVertex};

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
                const Y: usize = const_random!(usize) % 16;
                let packed = PackedVertex::<N, X, Y>::try_from((x, y));
                let x_too_big = x as u16 >= 1 << X;
                let y_too_big = y as u16 >= 1 << Y;
                if x_too_big {
                    assert!(packed == Err(Error::OversizedX));
                } else if y_too_big {
                    assert!(packed == Err(Error::OversizedY));
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
                const UNUSED: usize = const_random!(usize) % (BITS - X);
                const Y: usize = (BITS - X - UNUSED) % 16;
                let valid_x = (x % (1 << X)) as i16;
                let valid_y = (y % (1 << Y)) as i16;
                let packed = PackedVertex::<N, X, Y>::try_from((valid_x, valid_y));
                assert!(packed.is_ok());
                let (new_x, new_y) = packed.unwrap().unpack();
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
