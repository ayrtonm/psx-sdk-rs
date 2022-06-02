//! TIM file parsing
use crate::gpu::{Bpp, Clut, TexPage, Vertex, VertexError};

const MAGIC: u32 = 0x0000_0010;

/// A reference to a valid TIM file in memory.
#[derive(Debug)]
pub struct TIM<'a> {
    /// The TIM's bits per pixel.
    pub bpp: Bpp,
    /// The TIM bitmap data.
    pub bmp: Bitmap<'a, TexPage>,
    /// The TIM color lookup table data.
    pub clut_bmp: Option<Bitmap<'a, Clut>>,
}

/// Errors when parsing TIM files.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TIMError {
    /// The initial magic bytes are missing.
    MissingMagic,
    /// The initial magic bytes are wrong.
    BadMagic,
    /// The bits per pixel is missing.
    MissingBpp,
    /// The bits per pixel is invalid.
    InvalidBpp,
    /// The bitmap data is missing.
    MissingData,
    /// The color lookup table data is invalid.
    InvalidClut,
    /// The bitmap data is invalid.
    InvalidImage,
}

impl<'a> TIM<'a> {
    /// Parses and validates a TIM from a mutable `u32` slice.
    ///
    /// This modifies the in-memory TIM to make it easier to transfer to the GPU
    /// using DMA.
    pub fn new(src: &'a mut [u32]) -> Result<Self, TIMError> {
        if *src.get(0).ok_or(TIMError::MissingMagic)? != MAGIC {
            return Err(TIMError::BadMagic)
        }
        let flags = src.get(1).ok_or(TIMError::MissingBpp)?;
        let bpp = match flags & 0b11 {
            0 => Bpp::Bits4,
            1 => Bpp::Bits8,
            2 => Bpp::Bits15,
            _ => return Err(TIMError::InvalidBpp),
        };
        if src.len() < 3 {
            return Err(TIMError::MissingData)
        }
        let (clut_bmp, other) = if (flags & 8) != 0 {
            let (bmp, other) =
                Bitmap::new_clut(&mut src[2..]).map_err(|_| TIMError::InvalidClut)?;
            (Some(bmp), other)
        } else {
            (None, &mut src[2..])
        };
        let (bmp, _) = Bitmap::new_bmp(other).map_err(|_| TIMError::InvalidImage)?;
        Ok(TIM { bpp, bmp, clut_bmp })
    }
}

/// A bitmap which `TIM`s are composed of.
#[derive(Debug)]
pub struct Bitmap<'a, T: TryFrom<Vertex>>
where VertexError: From<<T as TryFrom<Vertex>>::Error> {
    /// The bitmap's offset in VRAM.
    pub offset: T,
    /// The size of the bitmap.
    pub size: Vertex,
    /// The bitmap data.
    pub data: &'a [u32],
}

impl<'a, T: TryFrom<Vertex>> Bitmap<'a, T>
where VertexError: From<<T as TryFrom<Vertex>>::Error>
{
    /// Creates a new bitmap for the `TIM` image data.
    pub fn new_bmp(src: &'a mut [u32]) -> Result<(Self, &'a mut [u32]), VertexError> {
        Self::new(src, 64, 256)
    }
    /// Creates a new bitmap for the `TIM` color lookup table data.
    pub fn new_clut(src: &'a mut [u32]) -> Result<(Self, &'a mut [u32]), VertexError> {
        Self::new(src, 16, 1)
    }
    fn new(
        src: &'a mut [u32], x_step: i16, y_step: i16,
    ) -> Result<(Self, &'a mut [u32]), VertexError> {
        let len = src[0] / 4;
        src[0] = 0xA0 << 24;
        let x = src[1] as i16;
        let y = (src[1] >> 16) as i16;
        let width = src[2] as i16;
        let height = (src[2] >> 16) as i16;
        let (data, other) = src.split_at_mut(len as usize);
        let bmp = Bitmap {
            offset: T::try_from(Vertex(x / x_step, y / y_step))?,
            size: Vertex(width, height),
            data,
        };
        Ok((bmp, other))
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
