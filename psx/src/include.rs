/// Gets an external file's size.
#[macro_export]
macro_rules! file_size {
    ($file:literal) => {{
        const N: usize = include_bytes!($file).len();
        N
    }};
}

/// Include an external file as a u8 array.
#[macro_export]
macro_rules! include_u8 {
    ($file:literal) => {{
        use $crate::file_size;
        const N: usize = file_size!($file);
        const AR: [u8; N] = *include_bytes!($file);
        AR
    }};
}

/// Include an external file as a u16 array.
#[macro_export]
macro_rules! include_u16 {
    ($file:literal) => {{
        use $crate::{file_size, include_u8};
        const N: usize = file_size!($file) / 2;
        const AR: [u16; N] = unsafe { core::mem::transmute(include_u8!($file)) };
        AR
    }};
}

/// Include an external file as a u32 array.
#[macro_export]
macro_rules! include_u32 {
    ($file:literal) => {{
        use $crate::{file_size, include_u8};
        const N: usize = file_size!($file) / 4;
        const AR: [u32; N] = unsafe { core::mem::transmute(include_u8!($file)) };
        AR
    }};
}

/// Gets a zipped file's decompressed size from its header.
#[macro_export]
macro_rules! unzipped_size {
    ($file:literal) => {{
        use $crate::include_u32;
        const N: usize = include_u32!($file)[0] as usize;
        N
    }};
}

/// Decompresses a zipped file.
#[macro_export]
macro_rules! unzip {
    ($file:literal) => {{
        use $crate::unzip::unzip;
        use $crate::{include_u32, unzipped_size};
        const N: usize = unzipped_size!($file);
        unsafe { unzip(include_u32!($file)) as [u32; N] }
    }};
}
