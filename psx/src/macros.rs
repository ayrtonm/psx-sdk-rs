#[doc(hidden)]
#[macro_export]
macro_rules! file_size {
    ($file:literal) => {{
        const N: usize = include_bytes!($file).len();
        N
    }};
}

/// Includes the specified file as a `[u32; N]`. This is exactly like
/// `include_bytes`, expect that it checks the size is a multiple of 4 bytes at
/// compile-time to allow creating a `u32` array rather than a `u8` array.
#[macro_export]
macro_rules! include_array {
    ($file:literal) => {{
        use $crate::file_size;
        const N: usize = file_size!($file) / 4;
        const _: () = {
            if N % 4 != 0 {
                panic!("File size is not a multiple of 4 bytes");
            }
        };
        const ret: [u32; N] = unsafe { core::mem::transmute(*include_bytes!($file)) };
        ret
    }};
}
