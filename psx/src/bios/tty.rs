use crate::std::AsCStr;
use core::fmt;

pub struct TTY;

/// Calls [A(3Fh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
#[macro_export]
macro_rules! printf {
    ($msg:expr $(,$args:expr)*) => {
        unsafe {
            $crate::bios::kernel::printf($msg.as_ptr() $(,$args)*)
        }
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            use $crate::bios::tty::TTY;
            <TTY as core::fmt::Write>::write_fmt(&mut TTY, format_args!($($arg)*)).ok();
        }
    };
}

// TODO: Use format_args_nl when it becomes stable
#[macro_export]
macro_rules! println {
    () => {
        $crate::printf!("\n\0");
    };
    ($($arg:tt)*) => {
        {
            use $crate::bios::tty::TTY;
            <TTY as core::fmt::Write>::write_fmt(&mut TTY, format_args!($($arg)*)).ok();
            $crate::printf!("\n\0");
        }
    };
}

impl fmt::Write for TTY {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.as_cstr(|s| printf!(s));
        Ok(())
    }
}
