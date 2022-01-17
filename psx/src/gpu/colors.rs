use crate::graphics::f16;
use crate::gpu::Color;

// This is the max value for untextured graphics. Colors for textured graphics
// should be scaled down to a max of 0x80.
const MAX: u8 = 0xFF;
pub const BLACK: Color = Color::new(0, 0, 0);
pub const WHITE: Color = Color::new(MAX, MAX, MAX);
pub const RED: Color = Color::new(MAX, 0, 0);
pub const GREEN: Color = Color::new(0, MAX, 0);
pub const BLUE: Color = Color::new(0, 0, MAX);

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

    pub const fn to_textured(&self) -> Self {
        self.halve()
    }

    pub const fn from_textured(&self) -> Self {
        self.double()
    }

    pub fn scale(&self, a: f16) -> Self {
        if a >= f16(0x1_000) {
            *self
        } else if a <= f16(0) {
            BLACK
        } else {
        let red = self.red * a;
        let green = self.green * a;
        let blue = self.blue * a;
        Color::new(red, green, blue)
        }
    }
}
