/// Gets a file size in bytes at compile-time.
#[macro_export]
macro_rules! file_size {
    ($file:literal) => {{
        const N: usize = include_bytes!($file).len();
        N
    }};
}

/// Includes the specified file as a `&mut [u32; N]`. This is exactly like
/// `include_bytes`, expect that it checks the size is a multiple of 4 bytes at
/// compile-time to allow creating a mutable `u32` slice rather than a `u8`
/// slice.
#[macro_export]
macro_rules! include_words {
    ($file:literal) => {{
        const N: usize = $crate::file_size!($file) / 4;
        const _: () = {
            if N % 4 != 0 {
                panic!("File size is not a multiple of 4 bytes");
            }
        };
        #[used]
        #[link_section = ".data"]
        static mut RET: [u32; N] = unsafe { core::mem::transmute(*include_bytes!($file)) };
        unsafe { &mut RET }
    }};
}
