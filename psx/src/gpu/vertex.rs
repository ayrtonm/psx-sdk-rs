// This represents x in [0, 1024) and y in [0, 512)
pub type Pixel = i16;

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

    pub fn map<F>(&self, f: F) -> Self
        where F: Fn((Pixel, Pixel)) -> (Pixel, Pixel) {
        f((self.x(), self.y())).into()
    }

    pub fn shift<T>(&self, v: T) -> Self
        where Vertex: From<T> {
        let v = Vertex::from(v);
        self.map(|(x, y)| (x + v.x(), y + v.y()))
    }

    pub fn rect<T, U>(center: T, size: U) -> Quad
        where Vertex: From<T> + From<U> {
        let center = Vertex::from(center);
        let size = Vertex::from(size);
        let half_size = size.map(|(x, y)| (x / 2, y / 2));
        [
            center.shift(&half_size.map(|(x, y)| (-x, -y))),
            center.shift(&half_size.map(|(x, y)| (-x, y))),
            center.shift(&half_size.map(|(x, y)| (x, -y))),
            center.shift(&half_size),
        ]
    }

    pub fn square<T>(center: T, length: Pixel) -> Quad
        where Vertex: From<T> {
        Vertex::rect::<T, (Pixel, Pixel)>(center, (length, length))
    }
}
