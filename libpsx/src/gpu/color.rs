//use core::num::FpCategory;
//use crate::util::Primitives;

type Component = u16;
#[derive(Clone, Copy, Default)]
//#[repr(packed(32))]
pub struct Color {
    red: Component,
    green: Component,
    blue: Component,
}

pub enum Opacity {
    Opaque,
    Translucent,
}

pub enum Palette<const N: usize> {
    Monochrome(Color),
    Shaded([Color; N]),
}


impl From<Color> for u32 {
    fn from(c: Color) -> u32 {
        (c.blue as u32) << 16 | (c.green as u32) << 8 | (c.red as u32)
    }
}

impl Color {
    pub fn new(red: Component, green: Component, blue: Component) -> Self {
        //red &= (1 << 8) - 1;
        //green &= (1 << 8) - 1;
        //blue &= (1 << 8) - 1;
        Color { red, green, blue }
    }
    pub fn red() -> Self { Color::new(255, 0, 0) }
    pub fn green() -> Self { Color::new(0, 255, 0) }
    pub fn blue() -> Self { Color::new(0, 0, 255) }

    pub fn yellow() -> Self { Color::red().sum(&Color::green()) }
    pub fn cyan() -> Self { Color::green().sum(&Color::blue()) }
    pub fn violet() -> Self { Color::blue().sum(&Color::red()) }

    pub fn orange() -> Self { Color::red().average(&Color::yellow()) }
    pub fn lime() -> Self { Color::green().average(&Color::yellow()) }
    pub fn mint() -> Self { Color::green().average(&Color::cyan()) }
    pub fn aqua() -> Self { Color::blue().average(&Color::cyan()) }
    pub fn indigo() -> Self { Color::blue().average(&Color::violet()) }
    pub fn pink() -> Self { Color::red().average(&Color::violet()) }

    pub fn black() -> Self { Color::new(0, 0, 0) }
    pub fn white() -> Self { Color::new(255, 255, 255) }

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
    // Halves the intensity of each component. This is preferred over `scale(0.5)`.
    pub fn halve(&self) -> Self {
        self.map(|c| c >> 1)
    }
    // Doubles the intensity of each component. This is preferred over `scale(2.0)`.
    pub fn double(&self) -> Self {
        self.map(|c| c << 1)
    }
    pub fn average(&self, other: &Self) -> Self {
        self.halve().sum(&other.halve())
    }
    // Scales each component by alpha. `halve()` and `double()` are preferred for powers of 2.
    //pub fn scale(&self, alpha: f32) -> Self {
    //    match alpha.classify() {
    //        FpCategory::Zero | FpCategory::Subnormal => Color::black(),
    //        FpCategory::Normal => {
    //            let log2alpha = alpha.log2();
    //            if log2alpha.fract() == 0.0 {
    //                let n = log2alpha.trunc() as i32;
    //                match n {
    //                    0 => *self,
    //                    1..=i32::MAX => (0..n).fold(*self, |color, _| color.double()),
    //                    i32::MIN..=-1 => (n..0).fold(*self, |color, _| color.halve()),
    //                }
    //            } else {
    //                self.map(|c| ((c as f32) * alpha) as Component)
    //            }
    //        },
    //        _ => Color::white(),
    //    }
    //}
}
