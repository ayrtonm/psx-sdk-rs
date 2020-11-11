type Component = u16;
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
}
