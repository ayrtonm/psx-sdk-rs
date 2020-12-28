use core::ops::{Add, Div, Mul, Sub};

/// An alias for a signed 16-bit integer.
pub type Pixel = i16;

/// A pair of coordinates.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GenericVertex<P: Copy> {
    /// The x coordinate.
    pub x: P,
    /// The y coordinate.
    pub y: P,
}

/// A pair of signed 16-bit coordinates.
pub type Vertex = GenericVertex<Pixel>;
/// A pair of signed 8-bit coordinates.
pub type SmallVertex = GenericVertex<i8>;

impl<P: Copy> From<P> for GenericVertex<P> {
    #[inline(always)]
    fn from(p: P) -> Self {
        (p, p).into()
    }
}

impl<P: Copy> From<(P, P)> for GenericVertex<P> {
    #[inline(always)]
    fn from((x, y): (P, P)) -> Self {
        GenericVertex { x, y }
    }
}

impl<P> GenericVertex<P>
where P: Copy + Add<Output = P> + Mul<Output = P> + Sub<Output = P> + Div<Output = P>
{
    /// Adds vertices component-wise.
    #[inline(always)]
    pub fn shift<T>(&self, other: T) -> Self
    where Self: From<T> {
        let other = GenericVertex::from(other);
        (self.x + other.x, self.y + other.y).into()
    }

    /// Multiplies vertices component-wise.
    #[inline(always)]
    pub fn scale<T>(&self, scale: T) -> Self
    where Self: From<T> {
        let scale = GenericVertex::from(scale);
        (self.x * scale.x, self.y * scale.y).into()
    }

    /// Subtracts vertices component-wise.
    #[inline(always)]
    pub fn subtract<T>(&self, other: T) -> Self
    where Self: From<T> {
        let other = GenericVertex::from(other);
        (self.x - other.x, self.y - other.y).into()
    }

    /// Divides vertices component-wise.
    #[inline(always)]
    pub fn divide<T>(&self, scale: T) -> Self
    where Self: From<T> {
        let scale = GenericVertex::from(scale);
        (self.x / scale.x, self.y / scale.y).into()
    }
}

/// A pair of `X` and `Y` bit coordinates packed into `N` bytes.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PackedVertex<const N: usize, const X: usize, const Y: usize> {
    data: [u8; N],
}

impl<T, const N: usize, const X: usize, const Y: usize> From<T> for PackedVertex<N, X, Y>
where Vertex: From<T>
{
    #[inline(always)]
    fn from(t: T) -> Self {
        let v = GenericVertex::from(t);
        let mut data = [0; N];
        let value = (v.x as u32) | ((v.y as u32) << X);
        data.copy_from_slice(&value.to_le_bytes()[0..N]);
        PackedVertex { data }
    }
}

impl<const X: usize, const Y: usize> PackedVertex<2, X, Y> {
    /// Converts a `PackedVertex` to a u32. The upper two bytes are guaranteed
    /// to be zero.
    pub fn as_u32(&self) -> u32 {
        self.data[0] as u32 | (self.data[1] as u32) << 8
    }
}

impl<const X: usize, const Y: usize> PackedVertex<3, X, Y> {
    /// Converts a `PackedVertex` to a u32. The upper byte is guaranteed to be
    /// zero.
    pub fn as_u32(&self) -> u32 {
        self.data[0] as u32 | (self.data[1] as u32) << 8 | (self.data[2] as u32) << 16
    }
}
