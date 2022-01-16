use crate::dma;
use crate::gpu::Bpp;
use crate::gpu::{Clut, TexPage, Vertex, VertexError};
use crate::hw::gpu::{GP0Command, GP0};
use strum_macros::IntoStaticStr;

const MAGIC: u32 = 0x0000_0010;

#[derive(Debug)]
pub struct TIM<'a> {
    bpp: Bpp,
    bmp: Bitmap<'a>,
    clut_bmp: Option<Bitmap<'a>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TIMCoords {
    pub tex_page: TexPage,
    pub clut: Option<Clut>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoStaticStr)]
pub enum TIMError {
    MissingMagic,
    BadMagic,
    MissingBpp,
    InvalidBpp,
    MissingData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoStaticStr)]
pub enum TIMLoadError {
    DMA(dma::Error),
    Vertex(VertexError),
}

impl<'a> TIM<'a> {
    pub fn new(src: &'a mut [u32]) -> Result<Self, TIMError> {
        if *src.get(0).ok_or(TIMError::MissingMagic)? != MAGIC {
            return Err(TIMError::BadMagic)
        }
        let flags = src.get(1).ok_or(TIMError::MissingBpp)?;
        let bpp = match flags & 0b11 {
            0 => Bpp::Bit4,
            1 => Bpp::Bit8,
            2 => Bpp::Bit15,
            _ => return Err(TIMError::InvalidBpp),
        };
        if src.len() < 3 {
            return Err(TIMError::MissingData)
        }
        let (clut_bmp, other) = if (flags & 8) != 0 {
            let (bmp, other) = Bitmap::new_clut(&mut src[2..]);
            (Some(bmp), other)
        } else {
            (None, &mut src[2..])
        };
        let (bmp, _) = Bitmap::new_bmp(other);
        Ok(TIM { bpp, bmp, clut_bmp })
    }

    pub fn load(&self, gpu_dma: Option<&mut dma::GPU>) -> Result<TIMCoords, VertexError> {
        // Bitmap::new writes 0xA000_0000 to the first word in each bmp slice
        // ensuring that they're valid GP0 commands. Since the slices reference
        // the command, CopyToVRAM itself doesn't have to be repr(C)
        struct CopyToVRAM<'a>(&'a [u32]);

        // Since the commands are dynamically sized we can't use the default
        // GP0Command impl
        impl GP0Command for CopyToVRAM<'_> {
            fn words(&self) -> &[u32] {
                self.0
            }
        }

        GP0::new().send_command(&CopyToVRAM(self.bmp.data));
        if let Some(clut) = &self.clut_bmp {
            GP0::new().send_command(&CopyToVRAM(clut.data));
        };
        // Can't use Option::map with the question mark operator
        let clut = match &self.clut_bmp {
            Some(clut) => Some(Clut::try_from(clut.offset)?),
            None => None,
        };
        Ok(TIMCoords {
            tex_page: TexPage::try_from(self.bmp.offset)?,
            clut,
        })
    }
}

#[derive(Debug)]
struct Bitmap<'a> {
    len: u32,
    offset: Vertex,
    size: Vertex,
    data: &'a [u32],
}

impl<'a> Bitmap<'a> {
    pub fn new_bmp(src: &'a mut [u32]) -> (Self, &'a mut [u32]) {
        Self::new(src, 64, 256)
    }
    pub fn new_clut(src: &'a mut [u32]) -> (Self, &'a mut [u32]) {
        Self::new(src, 16, 1)
    }
    fn new(src: &'a mut [u32], x_step: i16, y_step: i16) -> (Self, &'a mut [u32]) {
        let len = src[0] / 4;
        src[0] = 0xA0 << 24;
        let x = src[1] as i16;
        let y = (src[1] >> 16) as i16;
        let width = src[2] as i16;
        let height = (src[2] >> 16) as i16;
        let (data, other) = src.split_at_mut(len as usize);
        let bmp = Bitmap {
            len,
            offset: Vertex(x / x_step, y / y_step),
            size: Vertex(width, height),
            data,
        };
        (bmp, other)
    }
}

#[cfg(test)]
mod tests {

    use super::{TIMError, MAGIC, TIM};
    use aligned::{Aligned, A4};
    use core::mem::size_of;
    use core::slice;

    macro_rules! tim_test {
        ($tim:expr, $err:tt) => {
            let mut tim = $tim;
            assert!(TIM::new(&mut tim).unwrap_err() == TIMError::$err);
        };
    }

    #[test_case]
    fn bad_magic() {
        tim_test!([], MissingMagic);
        tim_test!([MAGIC + 1], BadMagic);
    }

    #[test_case]
    fn bad_bpp() {
        tim_test!([MAGIC], MissingBpp);
        tim_test!([MAGIC, 0x0000_0003], InvalidBpp);
    }

    #[test_case]
    fn no_data() {
        tim_test!([MAGIC, 0x0000_0000], MissingData);
        tim_test!([MAGIC, 0x0000_0001], MissingData);
        tim_test!([MAGIC, 0x0000_0002], MissingData);
    }

    #[test_case]
    fn minimal_tim() {
        let mut tim = [MAGIC, 0, 0, 0, 0];
        assert!(TIM::new(&mut tim).is_ok());
    }

    #[test_case]
    fn real_tim() {
        let mut font = Aligned::<A4, _>(*include_bytes!("../font.tim"));
        let tim = unsafe {
            slice::from_raw_parts_mut(font.as_mut_ptr() as *mut u32, font.len() / size_of::<u32>())
        };
        assert!(TIM::new(tim).is_ok());
    }
}
