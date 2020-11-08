#[derive(Clone, Copy, Default)]
#[repr(packed(32))]
pub struct Position {
    x: u16,
    y: u16,
}

pub type Polygon<const N: usize> = [Position; N];

impl From<Position> for u32 {
    fn from(p: Position) -> u32 {
        (p.y() as u32) << 16 | (p.x() as u32)
    }
}

impl Position {
    pub const fn new(x: u16, y: u16) -> Self {
        Position { x, y }
    }
    pub const fn x(&self) -> u16 {
        self.x
    }
    pub const fn y(&self) -> u16 {
        self.y
    }
    pub const fn zero() -> Self {
        Position::new(0, 0)
    }
    pub const fn rectangle(offset: Position, width: u16, height: u16) -> Polygon<4> {
        [offset,
         Position::new(offset.x() + width, offset.y()),
         Position::new(offset.x() + width, offset.y() + height),
         Position::new(offset.x(), offset.y() + height)]
    }
}
