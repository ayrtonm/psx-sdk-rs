use crate::gpu::AsU32;
use core::ops::AddAssign;
// This represents x in [0, 1024) and y in [0, 512)
pub type Pixel = i16;

#[derive(Clone, Copy)]
pub struct Vertex {
    x: Pixel,
    y: Pixel,
}

pub type Polygon<const N: usize> = [Vertex; N];

impl AsU32 for Vertex {
    fn as_u32(&self) -> u32 {
        (self.y as u32) << 16 | (self.x as u32)
    }
}

impl From<(Pixel, Pixel)> for Vertex {
    fn from((x, y): (Pixel, Pixel)) -> Vertex {
        Vertex { x, y }
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
    where F: Fn(Pixel, Pixel) -> (Pixel, Pixel) {
        f(self.x(), self.y()).into()
    }

    pub fn apply<F>(&mut self, f: F) -> &mut Self
    where F: Fn(Pixel, Pixel) -> (Pixel, Pixel) {
        *self = self.map(f);
        self
    }

    pub fn shift<T>(&self, v: T) -> Self
    where Vertex: From<T> {
        let v = Vertex::from(v);
        self.map(|x, y| (x + v.x(), y + v.y()))
    }

    pub fn rect<T, U>(center: T, size: U) -> Polygon<4>
    where Vertex: From<T> + From<U> {
        let center = Vertex::from(center);
        let size = Vertex::from(size);
        let half_size = size.map(|x, y| (x / 2, y / 2));
        [
            center.shift(half_size.map(|x, y| (-x, -y))),
            center.shift(half_size.map(|x, y| (-x, y))),
            center.shift(half_size.map(|x, y| (x, -y))),
            center.shift(half_size),
        ]
    }

    pub fn offset_rect<T, U>(offset: T, size: U) -> Polygon<4>
    where Vertex: From<T> + From<U> {
        let offset = Vertex::from(offset);
        let size = Vertex::from(size);
        [
            offset.clone(),
            offset.shift(size.map(|_, y| (0, y))),
            offset.shift(size.map(|x, _| (x, 0))),
            offset.shift(size),
        ]
    }

    pub fn square<T>(center: T, length: Pixel) -> Polygon<4>
    where Vertex: From<T> {
        Vertex::rect::<T, (Pixel, Pixel)>(center, (length, length))
    }

    pub fn offset_square<T>(offset: T, length: Pixel) -> Polygon<4>
    where Vertex: From<T> {
        Vertex::offset_rect::<T, (Pixel, Pixel)>(offset, (length, length))
    }
}

impl<T> AddAssign<T> for Vertex
where Vertex: From<T>
{
    fn add_assign(&mut self, other: T) {
        *self = self.shift(other);
    }
}
