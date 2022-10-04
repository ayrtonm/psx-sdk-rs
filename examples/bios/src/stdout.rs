use core::fmt;
use psx::sys::kernel::psx_std_out_putchar;

pub struct Stdout;

impl fmt::Write for Stdout {
    fn write_str(&mut self, msg: &str) -> fmt::Result {
        for c in msg.chars() {
            let ascii = if !c.is_ascii() { b'?' } else { c as u8 };
            unsafe {
                psx_std_out_putchar(ascii);
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! println {
    ($($args:tt)*) => {
        {
            use $crate::stdout::Stdout;
            <Stdout as core::fmt::Write>::write_fmt(&mut Stdout, format_args!($($args)*)).ok();
            unsafe {
                psx::sys::kernel::psx_std_out_putchar(b'\n');
            }
        }
    };
}
