//! TIM file parsing

use crate::gpu::{Bpp, Clut, TexPage, Vertex};
#[doc(hidden)]
pub const MAGIC: u32 = 0x0000_0010;

/// Validates and includes a [`TIM`][`crate::format::tim::TIM`] file.
#[macro_export]
macro_rules! include_tim {
    ($file:literal) => {{
        use core::mem::{size_of, transmute};
        use $crate::file_size;
        use $crate::format::tim::{Bitmap, MAGIC, TIM};
        use $crate::gpu::{Bpp, Clut, TexPage, Vertex};

        const TIM_SIZE: usize = (file_size!($file) + 3) / 4;
        const TIM_DATA: [u32; TIM_SIZE] = {
            let data = *include_bytes!($file);
            if data.len() % 4 != 0 {
                panic!("TIM size isn't a multiple of 4 bytes");
            }
            unsafe { transmute(data) }
        };
        const BPP: Bpp = {
            if TIM_DATA[0] != MAGIC {
                panic!("TIM file has invalid magic bytes");
            };
            match TIM_DATA[1] & 0b11 {
                0 => Bpp::Bits4,
                1 => Bpp::Bits8,
                2 => Bpp::Bits15,
                _ => panic!("TIM has invalid bpp"),
            }
        };
        const HAS_CLUT: bool = TIM_DATA[1] & 8 != 0;

        const CLUT_LEN: usize = if HAS_CLUT {
            TIM_DATA[2] as usize / size_of::<u32>()
        } else {
            0
        };

        const CLUT_INFO: (Clut, Vertex) = if HAS_CLUT {
            let offset = TIM_DATA[3];
            let x = offset as u16 as i16 / 16;
            let y = (offset >> 16) as u16 as i16;
            let clut = match Clut::const_try_from(Vertex(x, y)) {
                Ok(res) => res,
                Err(_) => panic!("TIM has invalid CLUT"),
            };
            let size = TIM_DATA[4];
            let x = size as u16;
            let y = (size >> 16) as u16;
            let size = Vertex(x as i16, y as i16);
            (clut, size)
        } else {
            let clut = match Clut::const_try_from(Vertex(0, 0)) {
                Ok(res) => res,
                Err(_) => panic!("Error in psx crate, this should be unreachable!"),
            };
            (clut, Vertex(0, 0))
        };

        static mut CLUT: Bitmap<Clut, CLUT_LEN> = Bitmap::<Clut, CLUT_LEN> {
            offset: CLUT_INFO.0,
            size: CLUT_INFO.1,
            data: {
                let mut clut = [0; CLUT_LEN];
                let mut i = 0;
                while i < CLUT_LEN {
                    clut[i] = TIM_DATA[2 + i];
                    i += 1;
                }
                clut[0] = 0xA0 << 24;
                clut
            },
        };
        const BMP_LEN: usize = TIM_DATA[2 + CLUT_LEN] as usize / size_of::<u32>();
        const BMP_INFO: (TexPage, Vertex) = {
            let offset = TIM_DATA[3 + CLUT_LEN];
            let x = offset as u16 as i16 / 64;
            let y = (offset >> 16) as u16 as i16 / 256;
            let tex_page = match TexPage::const_try_from(Vertex(x, y)) {
                Ok(res) => res,
                Err(_) => panic!("TIM has invalid bitmap TexPage"),
            };
            let size = TIM_DATA[4 + CLUT_LEN];
            let x = size as u16;
            let y = (size >> 16) as u16;
            let size = Vertex(x as i16, y as i16);
            (tex_page, size)
        };
        static mut BMP: Bitmap<TexPage, BMP_LEN> = Bitmap::<TexPage, BMP_LEN> {
            offset: BMP_INFO.0,
            size: BMP_INFO.1,
            data: {
                let mut bmp = [0; BMP_LEN];
                let mut i = 0;
                while i < BMP_LEN {
                    bmp[i] = TIM_DATA[2 + i + CLUT_LEN];
                    i += 1;
                }
                bmp[0] = 0xA0 << 24;
                bmp
            },
        };
        TIM::<BMP_LEN, CLUT_LEN> {
            bpp: BPP,
            bmp: unsafe { &mut BMP },
            clut: unsafe { &mut CLUT },
        }
    }};
}

/// A Reference to a TIM file in memory.
pub struct TIM<'a, const N: usize, const M: usize> {
    /// Bits per pixel
    pub bpp: Bpp,
    /// The TIM file's bitmap data
    pub bmp: &'a mut Bitmap<TexPage, N>,
    /// The TIM file's color lookup table bitmap data
    pub clut: &'a mut Bitmap<Clut, M>,
}

/// A bitmap which `TIM`s are composed of.
#[derive(Debug)]
pub struct Bitmap<T, const N: usize> {
    /// The bitmap's offset in VRAM.
    pub offset: T,
    /// The size of the bitmap.
    pub size: Vertex,
    /// The bitmap data.
    pub data: [u32; N],
}

#[cfg(test)]
mod tests {

    use crate::gpu::{Bpp, Clut, TexPage, Vertex};

    #[test_case]
    fn check_font() {
        let font = include_tim!("../../font.tim");
        assert!(font.bpp == Bpp::Bits4);
        assert!(font.clut.offset == Clut::try_from(Vertex(0, 480)).unwrap());
        assert!(font.clut.size == Vertex(16, 1));
        assert!(font.bmp.offset == TexPage::try_from(Vertex(10, 0)).unwrap());
        assert!(font.bmp.size == Vertex(32, 48));
    }
}
