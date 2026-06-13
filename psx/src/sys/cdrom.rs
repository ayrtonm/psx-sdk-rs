//! Raw CD-ROM access (via the BIOS functions).

use crate::sys::kernel;

enum Error {
    /// The output buffer is not divisible by the size of a CD-ROM Mode 1 sector
    /// (2048 bytes)
    BufferNotDivisible,
    /// An error occured during a BIOS function call. The value is the error
    /// code obtained from the BIOS.
    BIOSError(i32),
}

/// Reads Mode 1 sectors from the CD, starting from the sector at `start`.
///
/// The buffer must be exactly large enough to fit as many sectors to read from
/// the CD. (2 sectors: buffer of 4096 bytes, e.g.)
///
/// Returns the amount of sectors that were read from the CD.
pub fn read_sectors(start: i32, buffer: &mut [u8]) -> Result<usize, Error> {
    // Check if the size of the buffer is divisible by the CD sector size.
    if buffer.len() & 0x7FF != 0 {
        return Err(Error::BufferNotDivisible);
    }

    // SAFETY: We've checked the size of the output buffer to ensure that there
    // won't be an overflow when the BIOS reads sectors from the CD into the
    // buffer.
    let res = unsafe {
        kernel::psx_cd_read_sector((buffer.len() >> 11) as i32, start, buffer.as_mut_ptr())
    };

    match res {
        0..=i32::MAX => Ok(res as usize),
        i32::MIN..=-1 => Err(Error::BIOSError(res)),
    }
}
