type Intensity = u8;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color {
    red: Intensity,
    green: Intensity,
    blue: Intensity,
}

impl Color {
    pub fn rgb888(red: Intensity, green: Intensity, blue: Intensity) -> Self {
        Color { red, green, blue }
    }

    pub fn red() -> Self {
        Color::rgb888(255, 0, 0)
    }

    pub fn green() -> Self {
        Color::rgb888(0, 255, 0)
    }

    pub fn blue() -> Self {
        Color::rgb888(0, 0, 255)
    }

    pub fn black() -> Self {
        Color::rgb888(0, 0, 0)
    }

    pub fn white() -> Self {
        Color::rgb888(255, 255, 255)
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
        Color::rgb888(red, green, blue)
    }

    fn map<F>(&self, f: F) -> Self
    where F: Fn(Intensity) -> Intensity {
        Color::rgb888(f(self.red), f(self.green), f(self.blue))
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
