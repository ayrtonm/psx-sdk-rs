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
