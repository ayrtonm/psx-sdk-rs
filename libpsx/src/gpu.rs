use crate::bios;
use crate::util::{intercalate, prepend};

#[derive(Clone, Copy)]
pub struct Position {
    x: u16,
    y: u16,
}

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

pub enum Opacity {
    Opaque,
    Translucent,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Position { x, y }
    }
    pub fn x(&self) -> u16 {
        self.x
    }
    pub fn y(&self) -> u16 {
        self.y
    }
    pub fn zero() -> Self {
        Position::new(0, 0)
    }
    pub fn rectangle(offset: Position, width: u16, height: u16) -> [Position; 4] {
        [offset,
         Position::new(offset.x() + width, offset.y()),
         Position::new(offset.x(), offset.y() + height),
         Position::new(offset.x() + width, offset.y() + height)]
    }
}

impl From<&Position> for u32 {
    fn from(p: &Position) -> u32 {
        (p.y as u32) << 16 | (p.x as u32)
    }
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
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
        where F: Fn(u8) -> u8 {
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
    pub fn scale(&self, alpha: f32) -> Self {
        self.map(|c| ((c as f32) * alpha) as u8)
    }
}

impl From<&Color> for u32 {
    fn from(c: &Color) -> u32 {
        (c.blue as u32) << 16 | (c.green as u32) << 8 | (c.red as u32)
    }
}

pub enum Palette<const N: usize> {
    Monochrome(Color),
    Shaded([Color; N]),
}

// Draws generic triangles and rectangles. The palette can either be a color for each vertex or monochrome.
// TODO: find a better solution than const generics for the number of vertices or wait until they can be constrained to a set of values.
pub fn draw_polygon<const N: usize>(pos: &[Position; N], pal: &Palette<N>, opacity: &Opacity) {
    let mut v = match pal {
        Palette::Monochrome(color) => prepend(color, pos),
        Palette::Shaded(colors) => intercalate(colors, pos),
    };
    let cmd = match (N, pal, opacity) {
        (3, Palette::Monochrome(_), Opacity::Opaque) => 0x20,
        (3, Palette::Monochrome(_), Opacity::Translucent) => 0x22,
        (4, Palette::Monochrome(_), Opacity::Opaque) => 0x28,
        (4, Palette::Monochrome(_), Opacity::Translucent) => 0x2A,

        (3, Palette::Shaded(_), Opacity::Opaque) => 0x30,
        (3, Palette::Shaded(_), Opacity::Translucent) => 0x32,
        (4, Palette::Shaded(_), Opacity::Opaque) => 0x38,
        (4, Palette::Shaded(_), Opacity::Translucent) => 0x3A,
        (_, _, _) => todo!("remove this when rust can constrain const generics"),
    };
    v[0] |= cmd << 24;
    bios::gpu_command_word_params(&v);
}

// Draws rectangles of a given width and height. This is preferred for rectangles aligned to the screen.
pub fn draw_rect(offset: &Position, width: u16, height: u16, color: &Color, opacity: &Opacity) {
    enum SpecialRect { Pixel, Small, Medium };
    let special_size = match (width, height) {
        (1, 1) => Some(SpecialRect::Pixel),
        (8, 8) => Some(SpecialRect::Small),
        (16, 16) => Some(SpecialRect::Medium),
        _ => None,
    };
    let mut v = if special_size.is_some() {
        vec![u32::from(color), u32::from(offset)]
    } else {
        vec![u32::from(color), u32::from(offset), (height as u32) << 16 | width as u32]
    };
    let cmd = match (special_size, opacity) {
        (Some(SpecialRect::Pixel), Opacity::Opaque) => 0x68,
        (Some(SpecialRect::Small), Opacity::Opaque) => 0x70,
        (Some(SpecialRect::Medium), Opacity::Opaque) => 0x78,
        (None, Opacity::Opaque) => 0x60,
        (Some(SpecialRect::Pixel), Opacity::Translucent) => 0x6A,
        (Some(SpecialRect::Small), Opacity::Translucent) => 0x72,
        (Some(SpecialRect::Medium), Opacity::Translucent) => 0x7A,
        (None, Opacity::Translucent) => 0x62,
    };
    v[0] |= cmd << 24;
    bios::gpu_command_word_params(&v);
}
