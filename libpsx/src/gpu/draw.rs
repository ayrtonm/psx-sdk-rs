use crate::bios;
use crate::util::ArrayUtils;
use crate::{constrain, define, ret};
use crate::gpu::color::{Color, Palette, Opacity};
use crate::gpu::position::{Polygon, Position};

// FIXME: Remove this ugly hack when consts can be constrained in a sane way in rust.
// `constrain` is adapted from https://github.com/rust-lang/rust/issues/74674#issuecomment-662954029.
// While this gives a compile-time error if a constraint isn't satisfied, the parameters can't be
// inferred from the constraints. To avoid having to specify all paramters, the public API exposes
// only the required consts and uses where-clauses to implicitly add the constrained parameters to
// the public API. This is very restrictive though, so composing generic functions get's kinda ugly.
// See https://hackmd.io/OZG_XiLFRs2Xmw5s39jRzA for details.
pub fn line<const N: usize>(pos: &Polygon<N>, pal: &Palette<N>, opacity: Option<&Opacity>)
    where [(); N + 1]:,
          [(); N + 2]:,
          [(); N + N]:,
          [(); N + N + 1]: {
    let opacity = opacity.unwrap_or(&Opacity::Opaque);
    internal_line::<N, {N + 1}, {N + 2}, {N + N}, {N + N + 1}>(pos, pal, opacity);
}

fn internal_line<const N: usize, const M: usize, const O: usize, const P: usize, const Q: usize>(pos: &Polygon<N>, pal: &Palette<N>, opacity: &Opacity) {
    constrain!(N > 1);
    constrain!(M = N + 1);
    constrain!(O = N + 2);
    constrain!(P = N + N);
    constrain!(Q = P + 1);
    let cmd = match (N, pal, opacity) {
        (1, _, _) | (0, _, _) => unreachable!("Attempted to draw a one-vertex line"),
        (2, Palette::Monochrome(_), Opacity::Opaque) => 0x40,
        (2, Palette::Monochrome(_), Opacity::Translucent) => 0x42,
        (_, Palette::Monochrome(_), Opacity::Opaque) => 0x48,
        (_, Palette::Monochrome(_), Opacity::Translucent) => 0x4A,

        (2, Palette::Shaded(_), Opacity::Opaque) => 0x50,
        (2, Palette::Shaded(_), Opacity::Translucent) => 0x52,
        (_, Palette::Shaded(_), Opacity::Opaque) => 0x58,
        (_, Palette::Shaded(_), Opacity::Translucent) => 0x5A,
    };
    define!(arm := M, arp := P, aro := O, arq := Q);
    let ar = match (N, pal) {
        (2, Palette::Monochrome(color)) => {
            ret!(arm = pos.map(|p| p.into()).prepend((*color).into()))
        },
        (2, Palette::Shaded(colors)) => {
            ret!(arp = colors.map(|c| c.into()).intercalate(&pos.map(|p| p.into())))
        },
        (_, Palette::Monochrome(color)) => {
            ret!(aro = pos.map(|p| p.into()).prepend::<M>((*color).into()).append(0x5555_5555))
        },
        (_, Palette::Shaded(colors)) => {
            ret!(arq = {
                colors.map(|c| c.into())
                      .intercalate::<P>(&pos.map(|p| p.into()))
                      .append(0x5555_5555)
            })
        },
    };
    ar[0] |= cmd << 24;
    bios::gpu_command_word_params(&ar);
}

// FIXME: See `line`
pub fn frame<const N: usize>(pos: &Polygon<N>, pal: &Palette<N>, opacity: Option<&Opacity>)
    where [(); N + 1]:,
          [(); N + 2]:,
          [(); N + 3]:,
          [(); N + N + 2]:,
          [(); N + N + 3]: {
    let opacity = opacity.unwrap_or(&Opacity::Opaque);
    internal_frame::<N, {N + 1}, {N + 2}, {N + 3}, {N + N + 2}, {N + N + 3}>(pos, pal, opacity);
}

fn internal_frame<const N: usize, const M: usize, const O: usize, const P: usize, const Q: usize, const R: usize>(pos: &Polygon<N>, pal: &Palette<N>, opacity: &Opacity) {
    constrain!(N > 2);
    constrain!(M = N + 1);
    constrain!(O = M + 1);
    constrain!(P = M + 2);
    constrain!(Q = M + M);
    constrain!(R = Q + 1);
    let new_pos: &Polygon<M> = &pos.append(pos[0]);
    let new_pal = &match pal {
        Palette::Monochrome(c) => Palette::Monochrome(*c),
        Palette::Shaded(colors) => Palette::Shaded(colors.append(colors[0])),
    };
    internal_line::<M, O, P, Q, R>(new_pos, new_pal, opacity);
}

// FIXME: See `gpu::draw::line`
pub fn polygon<const N: usize>(pos: &Polygon<N>, pal: &Palette<N>, opacity: Option<&Opacity>)
    where [(); N + 1]:, [(); N + N]: {
    constrain!(N between 3 and 4);
    let opacity = opacity.unwrap_or(&Opacity::Opaque);
    let mut pos = *pos;
    if N == 4 {
        let temp_pos = pos[2];
        pos[2] = pos[3];
        pos[3] = temp_pos;
    }
    let pal = match pal {
        Palette::Monochrome(c) => Palette::Monochrome(*c),
        Palette::Shaded(colors) => {
            let mut colors = *colors;
            if N == 4 {
                let temp_col = colors[2];
                colors[2] = colors[3];
                colors[3] = temp_col;
            }
            Palette::Shaded(colors)
        },
    };
    polygon_low_level(&pos, &pal, opacity)
}

// FIXME: See `gpu::line`
pub fn polygon_low_level<const N: usize>(pos: &Polygon<N>, pal: &Palette<N>, opacity: &Opacity)
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
            ret!(ar1 = pos.map(|p| p.into()).prepend((*color).into()))
        },
        Palette::Shaded(colors) => {
            ret!(ar2 = colors.map(|c| c.into()).intercalate(&pos.map(|p| p.into())))
        },
    };
    ar[0] |= cmd << 24;
    bios::gpu_command_word_params(&ar);
}

/// Draws rectangles of a given width and height. This is preferred over `polygon`.
pub fn rect(offset: &Position, width: u16, height: u16, color: &Color, opacity: Option<&Opacity>) {
    let opacity = opacity.unwrap_or(&Opacity::Opaque);
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

pub fn triangle(pos: &Polygon<3>, pal: &Palette<3>, opacity: Option<&Opacity>) {
    let opacity = opacity.unwrap_or(&Opacity::Opaque);
    polygon_low_level(pos, pal, opacity)
}

pub fn quad(pos: &Polygon<4>, pal: &Palette<4>, opacity: Option<&Opacity>) {
    polygon(pos, pal, opacity)
}

/// Draws a square of a given length on each side. This is equivalent to `rect`.
pub fn square(offset: &Position, length: u16, color: &Color, opacity: Option<&Opacity>) {
    rect(offset, length, length, color, opacity)
}
