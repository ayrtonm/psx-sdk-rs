use core::prelude::*;

use core::fmt;

/// UART interface connected to the serial port, for debugging purposes
pub struct Uart;

impl Uart {
    pub fn new() -> Uart {
        Uart
    }

    /// Send a character using the BIOS `putc` function
    pub fn putchar(c: u8) {
        unsafe {
            bios_putchar(c)
        }
    }
}

/*impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &b in s.as_bytes() {
            Uart::putchar(b);
        }

        Ok(())
    }
}*/


extern {
    fn bios_putchar(b: u8);
}
