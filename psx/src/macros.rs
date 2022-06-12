/// Gets a file size in bytes at compile-time.
#[macro_export]
macro_rules! file_size {
    ($file:literal) => {{
        const N: usize = include_bytes!($file).len();
        N
    }};
}

/// Includes the specified file as a `&mut [u32; N]`.
///
/// This is exactly like `include_bytes`, expect that it pads the size to a
/// multiple of 4 bytes to allow creating a mutable `u32` slice rather than a
/// `u8` slice.
#[macro_export]
macro_rules! include_words {
    ($file:literal) => {{
        const N: usize = ($crate::file_size!($file) + 3) / 4;
        #[used]
        #[link_section = ".data"]
        static mut RET: [u32; N] = {
            let input_file = *include_bytes!($file);
            let mut padded_data = [0u8; N * 4];
            let mut i = 0;
            while i < input_file.len() {
                padded_data[i] = input_file[i];
                i += 1;
            }
            unsafe { core::mem::transmute(padded_data) }
        };
        unsafe { &mut RET }
    }};
}
