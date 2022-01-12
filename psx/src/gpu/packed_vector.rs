use crate::graphics::Vi;
use crate::gpu::{PackedVector, VectorError};
use core::convert::TryFrom;

impl<const N: usize, const X: usize, const Y: usize> PackedVector<N, X, Y> {
        const VALIDATE_X_PLUS_Y: () = {
            if X + Y > (N * 8) {
                panic!("Vector elements are larger than backing array");
            }
        };
}
impl<const X: usize, const Y: usize> From<PackedVector<2, X, Y>> for u32 {
    fn from(vector: PackedVector<2, X, Y>) -> u32 {
        vector.data[0] as u32 | (vector.data[1] as u32) << 8
    }
}

impl<const X: usize, const Y: usize> From<PackedVector<3, X, Y>> for u32 {
    fn from(vector: PackedVector<3, X, Y>) -> u32 {
        vector.data[0] as u32 | (vector.data[1] as u32) << 8 | (vector.data[2] as u32) << 16
    }
}

impl<const N: usize, const X: usize, const Y: usize> TryFrom<Vi> for PackedVector<N, X, Y> {
    type Error = VectorError;

    fn try_from(Vi(x, y): Vi) -> Result<Self, VectorError> {
        Self::VALIDATE_X_PLUS_Y;
        let x = x as u16;
        let y = y as u16;
        if x >= 1 << X {
            return Err(VectorError::InvalidX)
        }
        if y >= 1 << Y {
            return Err(VectorError::InvalidY)
        }
        let mut data = [0; N];
        let value = (x as u32 | ((y as u32) << X)).to_le_bytes();
        for i in 0..data.len() {
            data[i] = value[i];
        }
        Ok(PackedVector { data })
    }
}

impl<const N: usize, const X: usize, const Y: usize> PackedVector<N, X, Y> {
    pub fn unpack(&self) -> Vi {
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
        Vi(x as i16, y as i16)
    }
}

#[cfg(test)]
mod tests {

    use super::PackedVector;
    use crate::gpu::VectorError;
    use crate::graphics::Vi;

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
                const UNUSED: usize = const_random!(usize) % (BITS - X);
                const Y: usize = (BITS - X - UNUSED) % 16;
                let packed = PackedVector::<N, X, Y>::try_from(Vi(x, y));
                let x_too_big = x as u16 >= 1 << X;
                let y_too_big = y as u16 >= 1 << Y;
                if x_too_big {
                    assert!(packed == Err(VectorError::InvalidX));
                } else if y_too_big {
                    assert!(packed == Err(VectorError::InvalidY));
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
                let packed = PackedVector::<N, X, Y>::try_from(Vi(valid_x, valid_y));
                assert!(packed.is_ok());
                let Vi(new_x, new_y) = packed.unwrap().unpack();
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
