#[derive(Clone, Copy, Default)]
pub struct Position {
    x: u32,
    y: u32,
}

pub type Polygon<const N: usize> = [Position; N];

impl From<Position> for u32 {
    fn from(p: Position) -> u32 {
        (p.y() as u32) << 16 | (p.x() as u32)
    }
}

impl Position {
    pub const fn new(mut x: u32, mut y: u32) -> Self {
        //x &= (1 << 16) - 1;
        //y &= (1 << 16) - 1;
        Position { x, y }
    }
    pub const fn x(&self) -> u32 {
        self.x
    }
    pub const fn y(&self) -> u32 {
        self.y
    }
    pub const fn zero() -> Self {
        Position::new(0, 0)
    }
    pub const fn rect(offset: Position, width: u32, height: u32) -> Polygon<4> {
        [offset,
         Position::new(offset.x() + width, offset.y()),
         Position::new(offset.x() + width, offset.y() + height),
         Position::new(offset.x(), offset.y() + height)]
    }
}
