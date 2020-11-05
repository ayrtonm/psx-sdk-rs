use crate::bios;
use crate::util::ArrayUtils;
use crate::{constrain, define, ret};
use crate::gpu::color::{Color, Palette, Opacity};
use crate::gpu::position::Position;

pub fn draw_polygon<const N: usize>(pos: &[Position; N], pal: &Palette<N>, opacity: &Opacity)
    where [(); N + 1]:, [(); N + N]: {
    constrain!(N between 3 and 4);
    let mut pos = *pos;
    let temp_pos = pos[2];
    pos[2] = pos[3];
    pos[3] = temp_pos;
    let pal = match pal {
        Palette::Monochrome(c) => Palette::Monochrome(*c),
        Palette::Shaded(colors) => {
            let mut colors = *colors;
            let temp_col = colors[2];
            colors[2] = colors[3];
            colors[3] = temp_col;
            Palette::Shaded(colors)
        },
    };
    draw_polygon_ll(&pos, &pal, opacity)
}
pub fn draw_polygon_ll<const N: usize>(pos: &[Position; N], pal: &Palette<N>, opacity: &Opacity)
    where [(); N + 1]:, [(); N + N]: {
    constrain!(N between 3 and 4);
    let cmd = match (N, pal, opacity) {
        (3, Palette::Monochrome(_), Opacity::Opaque) => 0x20,
        (3, Palette::Monochrome(_), Opacity::Translucent) => 0x22,
        (4, Palette::Monochrome(_), Opacity::Opaque) => 0x28,
        (4, Palette::Monochrome(_), Opacity::Translucent) => 0x2A,

        (3, Palette::Shaded(_), Opacity::Opaque) => 0x30,
        (3, Palette::Shaded(_), Opacity::Translucent) => 0x32,
        (4, Palette::Shaded(_), Opacity::Opaque) => 0x38,
        (4, Palette::Shaded(_), Opacity::Translucent) => 0x3A,
        (n, _, _) => unreachable!("Attempted to draw {}-sided polygon", n),
    };
    define!(ar1 := N + 1, ar2 := N + N);
    let ar = match pal {
        Palette::Monochrome(color) => {
            ret!(ar1 = {
                pos.map(|p| p.into()).prepend((*color).into())
            })
        },
        Palette::Shaded(colors) => {
            ret!(ar2 = {
                colors.map(|c| c.into()).intercalate(&pos.map(|p| p.into()))
            })
        },
    };
    ar[0] |= cmd << 24;
    bios::gpu_command_word_params(&ar);
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
    let cmd = match (&special_size, opacity) {
        (Some(SpecialRect::Pixel), Opacity::Opaque) => 0x68,
        (Some(SpecialRect::Small), Opacity::Opaque) => 0x70,
        (Some(SpecialRect::Medium), Opacity::Opaque) => 0x78,
        (None, Opacity::Opaque) => 0x60,
        (Some(SpecialRect::Pixel), Opacity::Translucent) => 0x6A,
        (Some(SpecialRect::Small), Opacity::Translucent) => 0x72,
        (Some(SpecialRect::Medium), Opacity::Translucent) => 0x7A,
        (None, Opacity::Translucent) => 0x62,
    };
    let color = *color;
    let offset = *offset;
    define!(ar2 := 2, ar3 := 3);
    let ar = match special_size {
        Some(_) => ret!(ar2 = [color.into(), offset.into()]),
        None => ret!(ar3 = [color.into(), offset.into(), (height as u32) << 16 | width as u32]),
    };
    ar[0] |= cmd << 24;
    bios::gpu_command_word_params(&ar);
}

pub fn draw_square(offset: &Position, length: u16, color: &Color, opacity: &Opacity) {
    draw_rect(offset, length, length, color, opacity)
}
