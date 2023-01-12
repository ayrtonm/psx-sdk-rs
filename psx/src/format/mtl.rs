//! Wavefront MTL format importer
#![allow(missing_docs)]

use crate::gpu::Color;

#[doc(hidden)]
pub const fn count_colors(data: &[u8]) -> usize {
    let mut colors = 0;
    let mut i = 0;
    while i < data.len() {
        if data[i] == b'K' && data[i + 1] == b'd' {
            colors += 1;
            i += 1;
        } else {
            while i < data.len() && data[i] != b'\n' {
                i += 1;
            }
            if i < data.len() && data[i] == b'\n' {
                i += 1;
            }
        }
    }
    colors
}

#[derive(Debug)]
pub struct Mtl<'a, const N: usize> {
    pub colors: &'a mut [Color; N],
}

#[derive(Debug)]
pub struct MtlRef<'a> {
    pub colors: &'a [Color],
}

impl<const N: usize> Mtl<'_, N> {
    pub fn as_ref(&self) -> MtlRef {
        MtlRef {
            colors: self.colors,
        }
    }
}

#[macro_export]
macro_rules! include_mtl {
    ($file:literal) => {{
        use $crate::constants::BLACK;
        use $crate::format::mtl::{count_colors, Mtl};
        use $crate::format::parse_f16;
        use $crate::gpu::Color;
        const NUM_COLORS: usize = count_colors(include_bytes!($file));
        static mut COLORS: [Color; NUM_COLORS] = {
            let mut i = 0;
            let mut n = 0;
            let mtl = include_bytes!($file);
            let mut colors = [BLACK; NUM_COLORS];
            while i < mtl.len() {
                if mtl[i] == b'K' && mtl[i + 1] == b'd' {
                    i += 3;
                    let r = parse_f16(mtl, &mut i).to_bits() as u8;
                    let g = parse_f16(mtl, &mut i).to_bits() as u8;
                    let b = parse_f16(mtl, &mut i).to_bits() as u8;
                    colors[n] = Color::new(r, g, b);
                    n += 1;
                } else {
                    while i < mtl.len() && mtl[i] != b'\n' {
                        i += 1;
                    }
                    if i == mtl.len() {
                        break
                    }
                    if mtl[i] == b'\n' {
                        i += 1;
                    }
                }
            }
            colors
        };
        Mtl {
            colors: unsafe { &mut COLORS },
        }
    }};
}
