use crate::gpu::Bpp;
use strum_macros::IntoStaticStr;

const MAGIC: u32 = 0x0000_0010;

#[derive(Debug)]
pub struct TIM<'a> {
    bpp: Bpp,
    bmp: Bitmap<'a>,
    clut_bmp: Option<Bitmap<'a>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoStaticStr)]
pub enum Error {
    MissingMagic,
    BadMagic,
    MissingBpp,
    InvalidBpp,
    MissingData,
}

impl<'a> TIM<'a> {
    pub fn new(src: &'a mut [u32]) -> Result<Self, Error> {
        if *src.get(0).ok_or(Error::MissingMagic)? != MAGIC {
            return Err(Error::BadMagic)
        }
        let flags = src.get(1).ok_or(Error::MissingBpp)?;
        let bpp = match flags & 0b11 {
            0 => Bpp::Bit4,
            1 => Bpp::Bit8,
            2 => Bpp::Bit15,
            _ => return Err(Error::InvalidBpp),
        };
        if src.len() < 3 {
            return Err(Error::MissingData)
        }
        let (clut_bmp, other) = if (flags & 8) != 0 {
            let (bmp, other) = Bitmap::new(&mut src[2..]);
            (Some(bmp), other)
        } else {
            (None, &mut src[2..])
        };
        let (bmp, _) = Bitmap::new(other);
        Ok(TIM { bpp, bmp, clut_bmp })
    }
}

#[derive(Debug)]
struct Bitmap<'a>(&'a [u32]);

impl<'a> Bitmap<'a> {
    pub fn new(src: &'a mut [u32]) -> (Self, &'a mut [u32]) {
        let words = src[0] / 4;
        src[0] = 0xA0 << 24;
        let (data, other) = src.split_at_mut(words as usize);
        (Bitmap(data), other)
    }
}

#[cfg(test)]
mod tests {

    use super::{MAGIC, TIM, Error};

    macro_rules! tim_test {
        ($tim:expr, $err:tt) => {
            let mut tim = $tim;
            assert!(TIM::new(&mut tim).unwrap_err() == Error::$err);
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
        tim_test!([MAGIC, 0x0000_0002], MissingData);
        tim_test!([MAGIC, 0x0000_0002], MissingData);
    }

    #[test_case]
    fn minimal_tim() {
        let mut tim = [MAGIC, 0, 0];
        assert!(TIM::new(&mut tim).is_ok());
    }
}
