use super::kernel;
use core::pin::Pin;

type Buffer = [u8; 0x22];

/// Processes buffer data for the gamepads.
#[derive(Debug)]
pub struct GamePads<'a, 'b> {
    buf0: Pin<&'a mut Buffer>,
    buf1: Pin<&'b mut Buffer>,
}

impl<'a, 'b> GamePads<'a, 'b> {
    pub fn new(buf0: &'a mut Buffer, buf1: &'b mut Buffer) -> Self {
        let mut gp = Self {
            buf0: Pin::new(buf0),
            buf1: Pin::new(buf1),
        };
        unsafe {
            kernel::init_pad(
                gp.buf0.as_mut_ptr(),
                gp.buf0.len(),
                gp.buf1.as_mut_ptr(),
                gp.buf1.len(),
            );
            kernel::start_pad()
        }
        gp
    }
    pub fn stop(self) -> (Buffer, Buffer) {
        // impl Drop takes care of calling `stop_pad`
        (*self.buf0, *self.buf1)
    }
}

impl Drop for GamePads<'_, '_> {
    fn drop(&mut self) {
        unsafe { kernel::stop_pad() }
    }
}
