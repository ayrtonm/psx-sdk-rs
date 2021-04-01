use super::Color;

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
