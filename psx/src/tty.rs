use crate::std::AsCStr;
use core::fmt;

pub struct TTY;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        <TTY as core::fmt::Write>::write_fmt(&mut TTY, format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        printf!("\n\0")
    };
    ($($arg:tt)*) => {
        <TTY as core::fmt::Write>::write_fmt(&mut TTY, format_args_nl!($($arg)*))
    };
}

impl fmt::Write for TTY {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.as_cstr(|s| printf!(s));
        Ok(())
    }
}
