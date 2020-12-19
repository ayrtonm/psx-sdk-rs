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
        Vertex { x: p, y: p }
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
