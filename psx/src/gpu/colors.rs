use crate::gpu::Color;

pub const BLACK: Color = Color::new(0, 0, 0);
pub const WHITE: Color = Color::new(0xFF, 0xFF, 0xFF);
pub const RED: Color = Color::new(0xFF, 0, 0);
pub const GREEN: Color = Color::new(0, 0xFF, 0);
pub const BLUE: Color = Color::new(0, 0, 0xFF);

pub const YELLOW: Color = RED.sum(GREEN);
pub const CYAN: Color = GREEN.sum(BLUE);
pub const VIOLET: Color = BLUE.sum(RED);

pub const PINK: Color = RED.average(VIOLET);
pub const ORANGE: Color = RED.average(YELLOW);
pub const LIME: Color = GREEN.average(YELLOW);
pub const MINT: Color = GREEN.average(CYAN);
pub const AQUA: Color = BLUE.average(CYAN);
pub const INDIGO: Color = BLUE.average(VIOLET);

impl From<Color> for u32 {
    fn from(color: Color) -> u32 {
        color.red as u32 | (color.green as u32) << 8 | (color.blue as u32) << 16
    }
}

impl Color {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }

    pub const fn sum(&self, other: Self) -> Self {
        Color::new(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
        )
    }

    pub const fn halve(&self) -> Self {
        Color::new(self.red >> 1, self.green >> 1, self.blue >> 1)
    }

    pub const fn double(&self) -> Self {
        Color::new(self.red << 1, self.green << 1, self.blue << 1)
    }

    pub const fn average(&self, other: Self) -> Self {
        self.halve().sum(other.halve())
    }
}
