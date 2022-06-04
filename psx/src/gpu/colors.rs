#![allow(missing_docs)]
// No need for doc comments for each color.

use crate::gpu::{Color, TexColor};
use core::ops::{AddAssign, Div, DivAssign};

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

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, other: Color) {
        *self = self.sum(other);
    }
}

impl<T: Into<TexColor>> AddAssign<T> for TexColor {
    fn add_assign(&mut self, other: T) {
        *self = self.sum(other.into());
    }
}

impl Div<u8> for Color {
    type Output = Self;
    fn div(self, other: u8) -> Self {
        Self::new(self.red / other, self.green / other, self.blue / other)
    }
}
impl Div<u8> for TexColor {
    type Output = Self;
    fn div(self, other: u8) -> Self {
        Self::new(self.red / other, self.green / other, self.blue / other)
    }
}

impl DivAssign<u8> for Color {
    fn div_assign(&mut self, other: u8) {
        *self = *self / other;
    }
}

impl DivAssign<u8> for TexColor {
    fn div_assign(&mut self, other: u8) {
        *self = *self / other;
    }
}

impl Color {
    /// Creates a new `Color`.
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }

    /// Adds two `Color`s together.
    pub const fn sum(&self, other: Self) -> Self {
        Color::new(
            self.red.saturating_add(other.red),
            self.green.saturating_add(other.green),
            self.blue.saturating_add(other.blue),
        )
    }

    /// Halves a `Color`'s intensity, saturating to black.
    pub const fn halve(&self) -> Self {
        Color::new(self.red >> 1, self.green >> 1, self.blue >> 1)
    }

    /// Doubles a `Color`'s intensity, saturating to white.
    pub const fn double(&self) -> Self {
        Color::new(self.red << 1, self.green << 1, self.blue << 1)
    }

    /// Averages two `Color`s.
    pub const fn average(&self, other: Self) -> Self {
        self.halve().sum(other.halve())
    }
}

impl TexColor {
    /// Creates a new `TexColor`.
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    /// Adds two `TexColor`s together.
    pub const fn sum(&self, other: Self) -> Self {
        Self::new(
            self.red.saturating_add(other.red),
            self.green.saturating_add(other.green),
            self.blue.saturating_add(other.blue),
        )
    }
}

impl From<TexColor> for u32 {
    fn from(color: TexColor) -> u32 {
        color.red as u32 | (color.green as u32) << 8 | (color.blue as u32) << 16
    }
}

impl From<Color> for TexColor {
    fn from(Color { red, green, blue }: Color) -> TexColor {
        TexColor {
            red: red / 2,
            green: green / 2,
            blue: blue / 2,
        }
    }
}

impl From<TexColor> for Color {
    fn from(TexColor { red, green, blue }: TexColor) -> Color {
        Color {
            red: red * 2,
            green: green * 2,
            blue: blue * 2,
        }
    }
}
