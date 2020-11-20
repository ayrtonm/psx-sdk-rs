// This represents x in [0, 1024) and y in [0, 512)
pub type Pixel = i16;
// This isn't quite right either since the difference of unsigned 16-bit numbers
// can exceed an i16 but since valid values of x and y are restricted, it'll be
// fine in those cases. I'll keep this priivate anyway to avoid confusion.
type PixelDiff = i16;

pub struct Vertex {
    x: Pixel,
    y: Pixel,
}

pub type Polygon<const N: usize> = [Vertex; N];
pub type Line = Polygon<2>;
pub type Triangle = Polygon<3>;
pub type Quad = Polygon<4>;

impl From<(Pixel, Pixel)> for Vertex {
    fn from(v: (Pixel, Pixel)) -> Vertex {
        Vertex::new(v.0, v.1)
    }
}

impl From<&(Pixel, Pixel)> for Vertex {
    fn from(v: &(Pixel, Pixel)) -> Vertex {
        Vertex::new(v.0, v.1)
    }
}

impl From<Vertex> for u32 {
    fn from(v: Vertex) -> u32 {
        (v.y() as u32) << 16 | v.x() as u32
    }
}

impl From<&Vertex> for u32 {
    fn from(v: &Vertex) -> u32 {
        (v.y() as u32) << 16 | v.x() as u32
    }
}

// This is essentially Copy/Clone, but it's implemented as the `from` trait to
// make the DrawPort API more ergonomic while keeping ownership explicit. That
// is, `rect_to_vram(zero, ..)` will consume zero but `rect_to_vram(&zero, ..)`
// is also allowed. With Copy/Clone, `rect_to_vram(zero, ..)` would copy zero
// allowing it to be reused. In practice if the `Vertex` is converted to `u32`
// anyway, LTO optimizes that all out.
impl From<&Vertex> for Vertex {
    fn from(v: &Vertex) -> Vertex {
        Vertex::new(v.x(), v.y())
    }
}

impl Vertex {
    pub const fn new(x: Pixel, y: Pixel) -> Self {
        Vertex { x, y }
    }

    pub const fn x(&self) -> Pixel {
        self.x
    }

    pub const fn y(&self) -> Pixel {
        self.y
    }

    pub const fn zero() -> Self {
        Vertex::new(0, 0)
    }

    pub fn shrink_x(&self, a: Pixel) -> Self {
        Vertex::new(self.x() / a, self.y())
    }

    pub fn shrink_y(&self, a: Pixel) -> Self {
        Vertex::new(self.x(), self.y() / a)
    }

    pub fn scale_x(&self, a: Pixel) -> Self {
        Vertex::new(self.x() * a, self.y())
    }

    pub fn scale_y(&self, a: Pixel) -> Self {
        Vertex::new(self.x(), self.y() * a)
    }

    pub fn shift(&self, v: &Self) -> Self {
        Vertex::new(self.x() + v.x(), self.y() + v.y())
    }

    pub fn copy(&self) -> Self {
        Vertex::new(self.x(), self.y())
    }

    pub fn rect(center: &Vertex, size: Vertex) -> Quad {
        let half_size = size.shrink_x(2).shrink_y(2);
        [
            center.shift(&half_size.scale_x(-1).scale_y(-1)),
            center.shift(&half_size.scale_x(-1)),
            center.shift(&half_size.scale_y(-1)),
            center.shift(&half_size),
        ]
    }

    pub fn square(center: &Vertex, length: Pixel) -> Quad {
        Vertex::rect(center, Vertex::new(length, length))
    }
}
