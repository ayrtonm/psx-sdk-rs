type Intensity = u8;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub red: Intensity,
    pub green: Intensity,
    pub blue: Intensity,
}

impl Color {
    pub const AQUA: Self = { Color::BLUE.average(&Color::CYAN) };
    pub const BLACK: Self = { Color::rgb(0, 0, 0) };
    pub const BLUE: Self = { Color::rgb(0, 0, 255) };
    pub const CYAN: Self = { Color::GREEN.sum(&Color::BLUE) };
    pub const GREEN: Self = { Color::rgb(0, 255, 0) };
    pub const INDIGO: Self = { Color::BLUE.average(&Color::VIOLET) };
    pub const LIME: Self = { Color::GREEN.average(&Color::YELLOW) };
    pub const MINT: Self = { Color::GREEN.average(&Color::CYAN) };
    pub const ORANGE: Self = { Color::RED.average(&Color::YELLOW) };
    pub const PINK: Self = { Color::RED.average(&Color::VIOLET) };
    pub const RED: Self = { Color::rgb(255, 0, 0) };
    pub const VIOLET: Self = { Color::BLUE.sum(&Color::RED) };
    pub const WHITE: Self = { Color::rgb(255, 255, 255) };
    pub const YELLOW: Self = { Color::RED.sum(&Color::GREEN) };

    pub const fn rgb(red: Intensity, green: Intensity, blue: Intensity) -> Self {
        Color { red, green, blue }
    }

    pub const fn sum(&self, other: &Self) -> Self {
        let red = self.red + other.red;
        let green = self.green + other.green;
        let blue = self.blue + other.blue;
        Color::rgb(red, green, blue)
    }

    const fn from(c: [Intensity; 3]) -> Self {
        Color::rgb(c[0], c[1], c[2])
    }

    const fn to(&self) -> [Intensity; 3] {
        [self.red, self.green, self.blue]
    }

    // TODO: while loops are placeholder until `for` or array_map can be used in
    // const contexts Halves the intensity of each component.
    pub const fn halve(&self) -> Self {
        let mut ar = self.to();
        let mut i = 0;
        while i < ar.len() {
            ar[i] >>= 1;
            i += 1;
        }
        Color::from(ar)
    }

    // TODO: while loops are placeholder until `for` or array_map can be used in
    // const contexts Doubles the intensity of each component.
    pub const fn double(&self) -> Self {
        let mut ar = self.to();
        let mut i = 0;
        while i < ar.len() {
            ar[i] <<= 1;
            i += 1;
        }
        Color::from(ar)
    }

    pub const fn average(&self, other: &Self) -> Self {
        self.halve().sum(&other.halve())
    }
}
