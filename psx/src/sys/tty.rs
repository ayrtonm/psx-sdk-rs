#![doc(hidden)]
use crate::std::AsCStr;
/// Only the macros defined in this module are explicitly public-facing and
/// they're exported from the crate root.
use crate::sys::kernel;
use core::fmt;

pub struct TTY;

// TODO: Consider adding a feature for alloc to allow downcasting printf! args
// to String for implicit null-termination
/// Prints an ASCII string containing C-style escape codes to stdout.
///
/// This uses a single call to the BIOS `printf` function. The format string may
/// be any expression implementing `AsRef<[u8]>`.
#[macro_export]
macro_rules! printf {
    // This is the entry point of the macro
    ($msg:expr $(,$args:expr)*) => {
        {
            use core::any::Any;
            use $crate::std::AsCStr;
            use alloc::string::String;
            $msg.as_cstr(|cs| {
                printf!(@parse $($args,)*; {cs.as_ptr()});
            });
        }
    };
    (@parse $msg:expr $(,$args:expr)* $(,)?; {$($acc:tt)*}) => {
        {
            let any_ref = (&$msg as &dyn Any);
            if let Some(s) = any_ref.downcast_ref::<&[u8]>() {
                s.as_cstr(|cs| {
                    printf!(@parse $($args,)*; {$($acc)*, cs});
                })
            } else if let Some(s) = any_ref.downcast_ref::<&str>() {
                s.as_cstr(|cs| {
                    printf!(@parse $($args,)*; {$($acc)*, cs});
                })
            } else {
                printf!(@parse $($args,)*; {$($acc)*,$msg});
            }
        }
    };
    (@parse; {$($acc:tt)*}) => {
        unsafe {
            $crate::sys::kernel::printf($($acc)*)
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
            unsafe {
                $crate::sys::kernel::printf("\n\0".as_ptr());
            }
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
