type Pixel = i16;

/// A pair of signed 16-bit coordinates.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Vertex {
    /// The x coordinate.
    pub x: Pixel,
    /// The y coordinate.
    pub y: Pixel,
}

impl From<Pixel> for Vertex {
    #[inline(always)]
    fn from(p: Pixel) -> Self {
        (p, p).into()
    }
}

impl From<(Pixel, Pixel)> for Vertex {
    #[inline(always)]
    fn from((x, y): (Pixel, Pixel)) -> Self {
        Vertex { x, y }
    }
}

impl Vertex {
    /// Adds vertices component-wise.
    #[inline(always)]
    pub fn shift<T>(&self, other: T) -> Self
    where Vertex: From<T> {
        let other = Vertex::from(other);
        (self.x + other.x, self.y + other.y).into()
    }

    /// Multiplies vertices component-wise.
    #[inline(always)]
    pub fn scale<T>(&self, scale: T) -> Self
    where Vertex: From<T> {
        let scale = Vertex::from(scale);
        (self.x * scale.x, self.y * scale.y).into()
    }

    /// Divides vertices component-wise.
    #[inline(always)]
    pub fn divide<T>(&self, scale: T) -> Self
    where Vertex: From<T> {
        let scale = Vertex::from(scale);
        (self.x / scale.x, self.y / scale.y).into()
    }
}

/// A pair of coordinates packed into `X + Y <= 24` bits.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PackedVertex<const X: usize, const Y: usize> {
    data: [u8; 3],
}

impl<T, const X: usize, const Y: usize> From<T> for PackedVertex<X, Y>
where Vertex: From<T>
{
    #[inline(always)]
    fn from(t: T) -> Self {
        let v = Vertex::from(t);
        let mut data = [0; 3];
        let value = (v.x as u32) | ((v.y as u32) << X);
        data.copy_from_slice(&value.to_le_bytes()[0..3]);
        PackedVertex { data }
    }
}

impl<const X: usize, const Y: usize> PackedVertex<X, Y> {
    /// Converts a `PackedVertex` to a u32. The upper byte is guaranteed to be
    /// zero.
    pub fn as_u32(&self) -> u32 {
        self.data[0] as u32 | (self.data[1] as u32) << 8 | (self.data[2] as u32) << 16
    }
}
