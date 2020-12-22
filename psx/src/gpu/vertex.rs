pub type Pixel = i16;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Vertex {
    x: Pixel,
    y: Pixel,
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
    #[inline(always)]
    pub fn x(&self) -> Pixel {
        self.x
    }

    #[inline(always)]
    pub fn y(&self) -> Pixel {
        self.y
    }

    pub fn shift<T>(&self, other: T) -> Self
    where Vertex: From<T> {
        let other = Vertex::from(other);
        (self.x() + other.x(), self.y() + other.y()).into()
    }

    pub fn scale<T>(&self, scale: T) -> Self
    where Vertex: From<T> {
        let scale = Vertex::from(scale);
        (self.x() * scale.x(), self.y() * scale.y()).into()
    }
}

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
        //#[cfg(feature = "pretty_panic")]
        //{
        //    // TODO: This should be evaluated at compile-time
        //    if !(X + Y < 24) {
        //        panic!("Invalid PackedVertex");
        //    }
        //    if v.x() & !((1 << X) - 1) != 0 {
        //        panic!("Invalid x-coordinate in PackedVertex");
        //    }
        //    if v.y() & !((1 << Y) - 1) != 0 {
        //        panic!("Invalid y-coordinate in PackedVertex");
        //    }
        //}
        let mut data = [0; 3];
        let value = (v.x() as u32) | ((v.y() as u32) << X);
        data.copy_from_slice(&value.to_le_bytes()[0..3]);
        PackedVertex { data }
    }
}
