type Component = u8;
pub struct Color {
    red: Component,
    green: Component,
    blue: Component,
}

pub type Palette<'a, const N: usize> = &'a [Color; N];

impl From<&Color> for u32 {
    fn from(color: &Color) -> u32 {
        (color.blue as u32) << 16 | (color.green as u32) << 8 | (color.red as u32)
    }
}

impl Color {
    pub const fn new(red: Component, green: Component, blue: Component) -> Self {
        Color { red, green, blue }
    }

    pub const fn red() -> Self {
        Color::new(255, 0, 0)
    }

    pub const fn green() -> Self {
        Color::new(0, 255, 0)
    }

    pub const fn blue() -> Self {
        Color::new(0, 0, 255)
    }

    pub const fn black() -> Self {
        Color::new(0, 0, 0)
    }

    pub const fn white() -> Self {
        Color::new(255, 255, 255)
    }

    pub fn yellow() -> Self {
        Color::red().sum(&Color::green())
    }

    pub fn cyan() -> Self {
        Color::green().sum(&Color::blue())
    }

    pub fn violet() -> Self {
        Color::blue().sum(&Color::red())
    }

    pub fn orange() -> Self {
        Color::red().average(&Color::yellow())
    }

    pub fn lime() -> Self {
        Color::green().average(&Color::yellow())
    }

    pub fn mint() -> Self {
        Color::green().average(&Color::cyan())
    }

    pub fn aqua() -> Self {
        Color::blue().average(&Color::cyan())
    }

    pub fn indigo() -> Self {
        Color::blue().average(&Color::violet())
    }

    pub fn pink() -> Self {
        Color::red().average(&Color::violet())
    }

    pub fn sum(&self, other: &Self) -> Self {
        let red = self.red + other.red;
        let green = self.green + other.green;
        let blue = self.blue + other.blue;
        Color::new(red, green, blue)
    }

    fn map<F>(&self, f: F) -> Self
    where F: Fn(Component) -> Component {
        Color::new(f(self.red), f(self.green), f(self.blue))
    }

    // Halves the intensity of each component.
    pub fn halve(&self) -> Self {
        self.map(|c| c >> 1)
    }

    // Doubles the intensity of each component.
    pub fn double(&self) -> Self {
        self.map(|c| c << 1)
    }

    pub fn average(&self, other: &Self) -> Self {
        self.halve().sum(&other.halve())
    }
}
