pub(crate) type Pixel = i16;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    x: Pixel,
    y: Pixel,
}

impl From<(Pixel, Pixel)> for Vertex {
    fn from((x, y): (Pixel, Pixel)) -> Self {
        Vertex { x, y }
    }
}

impl Vertex {
    pub fn x(&self) -> Pixel {
        self.x
    }

    pub fn y(&self) -> Pixel {
        self.y
    }
}
