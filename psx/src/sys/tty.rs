#![doc(hidden)]
use crate::std::AsCStr;
/// Only the macros defined in this module are explicitly public-facing and
/// they're exported from the crate root.
use crate::sys::kernel;
use core::fmt;

pub struct TTY;

/// Prints an ASCII string containing C-style escape codes to stdout.
///
/// This uses a single call to the BIOS `printf` function. The format string may
/// be any expression implementing `AsRef<[u8]>`.
///
/// # Safety
///
/// Any string arguments formatted with `%s` must be explicitly null-terminated,
/// otherwise **undefined behavior** occurs. For example:
///
/// ```
/// // Explicit null-termination required since `world` is formatted with `%s`
/// let world = "world\0";
/// // The format string itself is implicitly null-terminated
/// printf!("hello %s!", world);
/// ```
#[macro_export]
macro_rules! printf {
    ($msg:expr $(,$args:tt)*) => {
        {
            use $crate::std::AsCStr;
            $msg.as_cstr(|cstr| {
                // SAFETY: the first argument is null-terminated.
                unsafe {
                    $crate::sys::kernel::printf(cstr.as_ptr() $(,$args)*)
                }
            })
        }
    };
}

/// Prints as ASCII string containing Rust-style escape codes to stdout.
///
/// This may call into the BIOS multiple times. The format string must be a
/// literal.
#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {
        {
            use $crate::sys::tty::TTY;
            <TTY as core::fmt::Write>::write_fmt(&mut TTY, format_args!($($args)*)).ok();
        }
    };
}

/// Prints as ASCII string containing Rust-style escape codes to stdout with a
/// newline.
///
/// This may call into the BIOS multiple times. The format string must be a
/// literal.
#[macro_export]
macro_rules! println {
    ($($args:tt)*) => {
        {
            use $crate::sys::tty::TTY;
            <TTY as core::fmt::Write>::write_fmt(&mut TTY, format_args!($($args)*)).ok();
            $crate::printf!("\n");
        }
    };
}

impl fmt::Write for TTY {
    fn write_str(&mut self, msg: &str) -> fmt::Result {
        msg.as_cstr(|cstr|
            // SAFETY: The format string and string argument are both null-terminated.
            unsafe {
                kernel::printf("%s\0".as_ptr(), cstr.as_ptr());
            });
        Ok(())
    }
}
