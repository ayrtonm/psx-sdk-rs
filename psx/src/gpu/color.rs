type Component = u8;

/// A triple of unsigned 8-bit color components.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color {
    /// The red component.
    pub red: Component,
    /// The green component.
    pub green: Component,
    /// The blue component.
    pub blue: Component,
}

impl From<(Component, Component, Component)> for Color {
    fn from((r, g, b): (Component, Component, Component)) -> Self {
        Color::rgb(r, g, b)
    }
}

#[allow(missing_docs)]
impl Color {
    pub const BLACK: Self = Color::rgb(0, 0, 0);
    pub const WHITE: Self = Color::rgb(0xFF, 0xFF, 0xFF);
    pub const RED: Self = Color::rgb(0xFF, 0, 0);
    pub const GREEN: Self = Color::rgb(0, 0xFF, 0);
    pub const BLUE: Self = Color::rgb(0, 0, 0xFF);

    pub const YELLOW: Self = Color::RED.sum(&Color::GREEN);
    pub const CYAN: Self = Color::GREEN.sum(&Color::BLUE);
    pub const VIOLET: Self = Color::BLUE.sum(&Color::RED);

    pub const PINK: Self = Color::RED.average(&Color::VIOLET);
    pub const ORANGE: Self = Color::RED.average(&Color::YELLOW);
    pub const LIME: Self = Color::GREEN.average(&Color::YELLOW);
    pub const MINT: Self = Color::GREEN.average(&Color::CYAN);
    pub const AQUA: Self = Color::BLUE.average(&Color::CYAN);
    pub const INDIGO: Self = Color::BLUE.average(&Color::VIOLET);

    /// Constructs a new color.
    pub const fn rgb(red: Component, green: Component, blue: Component) -> Self {
        Color { red, green, blue }
    }

    /// Adds two colors component-wise.
    pub const fn sum(&self, other: &Self) -> Self {
        Color::rgb(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
        )
    }

    /// Halves each component of a color.
    pub const fn halve(&self) -> Self {
        Color::rgb(self.red >> 1, self.green >> 1, self.blue >> 1)
    }

    /// Averages two colors.
    pub const fn average(&self, other: &Self) -> Self {
        self.halve().sum(&other.halve())
    }
}
