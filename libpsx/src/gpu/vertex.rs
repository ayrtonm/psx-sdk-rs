type Component = i16;
pub struct Vertex {
    x: Component,
    y: Component,
}

pub type Polygon<'a, const N: usize> = &'a [Vertex; N];
pub type Point<'a> = &'a Vertex;
pub type Line<'a> = Polygon<'a, 2>;
pub type PolyLine<'a> = &'a [Vertex];
pub type Length = Component;

impl From<&Vertex> for u32 {
    fn from(vertex: &Vertex) -> u32 {
        (vertex.y() as u32) << 16 | (vertex.x() as u32)
    }
}

impl Vertex {
    pub const fn new(x: Component, y: Component) -> Self {
        Vertex { x, y }
    }

    pub const fn x(&self) -> Component {
        self.x
    }

    pub const fn y(&self) -> Component {
        self.y
    }

    pub const fn zero() -> Self {
        Vertex::new(0, 0)
    }

    pub fn shift(&self, x: Component, y: Component) -> Self {
        Vertex::new(self.x() + x, self.y() + y)
    }

    pub fn copy(&self) -> Self {
        Vertex::new(self.x(), self.y())
    }

    // TODO: Change definition of `Polygon` to make this more consistent
    pub fn rect(center: &Vertex, width: Component, height: Component) -> [Vertex; 4] {
        let hw = width >> 1;
        let hh = height >> 1;
        [
            center.shift(-hw, -hh),
            center.shift(-hw, hh),
            center.shift(hw, -hh),
            center.shift(hw, hh),
        ]
        //[offset.copy(), offset.shift(width, 0), offset.shift(0, height),
        //[offset.copy(), offset.shift(width, height)]
    }

    // TODO: see `Vertex::rect`
    pub fn square(center: &Vertex, length: Component) -> [Vertex; 4] {
        Vertex::rect(center, length, length)
    }
}
