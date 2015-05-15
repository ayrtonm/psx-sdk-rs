use core::prelude::*;

use core::fmt;

/// UART interface connected to the serial port, for debugging purposes
pub struct Uart;

impl Uart {
    pub fn new() -> Uart {
        Uart
    }

    /// Send a character using the BIOS `putc` function
    pub fn putc(&self, c: u8) {
        unsafe {
            asm!("j 0xa0\n\t\
                  nop"
                 :
                 : "{$4}"(c as u32), "{$9}"(0x3c)
                 :
                 : );
        }
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &b in s.as_bytes() {
            self.putc(b);
        }

        Ok(())
    }
}
