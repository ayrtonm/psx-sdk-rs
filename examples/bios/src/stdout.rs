use core::ffi::CStr;
use core::fmt;
use psx::sys::kernel::psx_std_out_putchar;

pub struct Stdout;

// TODO: core::fmt adds about 10-20K by itself so I should consider ufmt or
// other fmt-free alternatives
impl fmt::Write for Stdout {
    fn write_str(&mut self, msg: &str) -> fmt::Result {
        for c in msg.chars() {
            let ascii = if !c.is_ascii() { b'?' } else { c as u8 };
            putchar(ascii);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {
        {
            use $crate::stdout::Stdout;
            <Stdout as core::fmt::Write>::write_fmt(&mut Stdout, format_args!($($args)*)).ok();
        }
    };
}
#[macro_export]
macro_rules! println {
    ($($args:tt)*) => {
        {
            $crate::print!($($args)*);
            $crate::stdout::putchar(b'\n');
        }
    };
}

pub fn putchar(c: u8) {
    unsafe { psx_std_out_putchar(c) }
}

pub fn printf(fmt_str: &CStr, arg0: u32, arg1: u32, arg2: u32) -> u32 {
    let args = [arg0, arg1, arg2];
    let mut va_arg = None;
    let mut args_used = 0;
    for &b in fmt_str.to_bytes() {
        if b == b'%' {
            if va_arg.is_none() {
                va_arg = Some(args_used);
                continue
            } else {
                va_arg = None;
            }
        }
        match va_arg {
            Some(idx) => {
                args_used += 1;
                match b {
                    b'c' => print!("{}", args[idx] as u8 as char),
                    b'd' | b'i' => print!("{}", args[idx] as i32),
                    b'u' => print!("{}", args[idx]),
                    b'x' => print!("{:x}", args[idx]),
                    b'X' => print!("{:X}", args[idx]),
                    b'p' => print!("{:p}", args[idx] as *const ()),
                    b'f' => print!("{}", args[idx] as f32),
                    b's' => {
                        // SAFETY: Let's hope the user passed in a null-terminated string
                        let str_arg = unsafe { CStr::from_ptr(args[idx] as *const i8) };
                        for &c in str_arg.to_bytes() {
                            putchar(c);
                        }
                    },
                    _ => args_used -= 1,
                }
                va_arg = None;
            },
            None => putchar(b),
        }
    }
    0
}
