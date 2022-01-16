#![doc(hidden)]
/// This module is hidden because only the macros defined in this module are
/// explicitly public-facing and they're exported from the crate root.
use crate::std::AsCStr;
use crate::sys::kernel;
use core::fmt;

pub trait PrintfString {
    fn impls_as_cstr(&self) -> Option<&[u8]>;
}

// min_specialization currently doesn't allow an impl<T: AsRef<[u8]>> so we have
// to enumerate the possible CStr types. This approach is better than the
// previous "if downcast_ref" in the macro because it supports all `[u8; N]`
impl<const N: usize> PrintfString for [u8; N] {
    fn impls_as_cstr(&self) -> Option<&[u8]> {
        Some(self.as_ref())
    }
}
impl PrintfString for &[u8] {
    fn impls_as_cstr(&self) -> Option<&[u8]> {
        Some(self.as_ref())
    }
}
impl PrintfString for &str {
    fn impls_as_cstr(&self) -> Option<&[u8]> {
        Some(self.as_ref())
    }
}

impl<T> PrintfString for T {
    default fn impls_as_cstr(&self) -> Option<&[u8]> {
        None
    }
}

pub struct TTY;

/// Prints an ASCII string containing C-style escape codes to stdout.
///
/// This uses a single call to the BIOS `printf` function. The format string may
/// be any expression implementing `AsRef<[u8]>`.
#[macro_export]
macro_rules! printf {
    // This is the entry point of the macro
    ($msg:expr $(,$args:expr)*) => {
        {
            use $crate::std::AsCStr;
            use $crate::sys::tty::PrintfString;
            $msg.as_cstr(|cs| {
                printf!(@parse $($args,)*; {cs.as_ptr()});
            });
        }
    };
    (@parse $msg:expr $(,$args:expr)* $(,)?; {$($acc:tt)*}) => {
        {
            if let Some(s) = $msg.impls_as_cstr() {
                s.as_cstr(|cs| {
                    printf!(@parse $($args,)*; {$($acc)*, cs.as_ptr()});
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn impls_as_cstr() {
        let array: [u8; 0] = [];
        let array_ref: &[u8; 0] = b"";
        let slice: &[u8] = &array_ref[..];
        let str_ref: &str = "";
        assert!(array.impls_as_cstr().is_some());
        assert!(array_ref.impls_as_cstr().is_some());
        assert!(slice.impls_as_cstr().is_some());
        assert!(str_ref.impls_as_cstr().is_some());
        assert!(0xdeadbeefu32.impls_as_cstr().is_none());
    }
}
