pub type Component = u16;
type SignedComponent = i16;
pub struct Vertex {
    x: Component,
    y: Component,
}

type Polygon<const N: usize> = [Vertex; N];
pub type Line = Polygon<2>;
pub type Triangle = Polygon<3>;
pub type Quad = Polygon<4>;

pub type PolyLine<'a> = &'a [Vertex];

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

    pub fn shift(&self, x: SignedComponent, y: SignedComponent) -> Self {
        let new_x = (self.x() as SignedComponent) + x;
        let new_y = (self.y() as SignedComponent) + y;
        Vertex::new(new_x as Component, new_y as Component)
    }

    pub fn copy(&self) -> Self {
        Vertex::new(self.x(), self.y())
    }

    pub fn rect(center: &Vertex, width: Component, height: Component) -> Quad {
        let hw = (width >> 1) as SignedComponent;
        let hh = (height >> 1) as SignedComponent;
        [
            center.shift(-hw, -hh),
            center.shift(-hw, hh),
            center.shift(hw, -hh),
            center.shift(hw, hh),
        ]
    }

    pub fn square(center: &Vertex, length: Component) -> Quad {
        Vertex::rect(center, length, length)
    }
}
