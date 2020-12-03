#[macro_export]
macro_rules! file_size {
    ($file:literal) => {{
        const N: usize = include_bytes!($file).len();
        N
    }};
}

#[macro_export]
macro_rules! include_u8 {
    ($file:literal) => {{
        use $crate::file_size;
        const N: usize = file_size!($file);
        const ar: [u8; N] = *include_bytes!($file);
        ar
    }};
}

#[macro_export]
macro_rules! include_u16 {
    ($file:literal) => {{
        use $crate::{file_size, include_u8};
        const N: usize = file_size!($file) / 2;
        const ar: [u16; N] = unsafe { core::mem::transmute(include_u8!($file)) };
        ar
    }};
}

#[macro_export]
macro_rules! include_u32 {
    ($file:literal) => {{
        use $crate::{file_size, include_u8};
        const N: usize = file_size!($file) / 4;
        const ar: [u32; N] = unsafe { core::mem::transmute(include_u8!($file)) };
        ar
    }};
}

#[macro_export]
macro_rules! unzipped_size {
    ($file:literal) => {{
        use $crate::include_u32;
        const N: usize = include_u32!($file)[0] as usize;
        N
    }};
}

// Calling decompress directly requires us to specify the decompressed size
// This can't be inferred, but exists in the zip file header so we read it as a
// const so it'll look like inference to the user
#[macro_export]
macro_rules! unzip_now {
    ($file:literal) => {{
        use $crate::unzip::unzip;
        use $crate::{include_u32, unzipped_size};
        const N: usize = unzipped_size!($file);
        unzip(include_u32!($file)) as [u32; N]
    }};
}

// Calling `unzip_now` unzips immediately. It may be better to defer
// decompression until the data is accessed. This requires the feature
// `once_cell`.
#[macro_export]
macro_rules! unzip {
    ($file:literal) => {{
        use core::lazy::Lazy;
        use $crate::{unzip_now, unzipped_size};
        const N: usize = unzipped_size!($file);
        Lazy::<[u32; N]>::new(|| unzip_now!($file))
    }};
}

// Lazily loads a tim. The closure in the Lazy constructor is not allowed to
// capture anything since it's being coerced to an fn pointer. This is why it
// has to be a macro instead of a function.
#[macro_export]
macro_rules! tim {
    ($data:expr) => {{
        use core::borrow::Borrow;
        use core::lazy::Lazy;
        use $crate::tim::TIM;
        trait MaybeLazy<T> {
            fn maybe_lazy(&self) -> &T;
        }
        impl<T> MaybeLazy<T> for Lazy<T> {
            fn maybe_lazy(&self) -> &T {
                self.borrow()
            }
        }
        impl<T: Borrow<[u32]>> MaybeLazy<T> for T {
            fn maybe_lazy(&self) -> &T {
                &self
            }
        }
        Lazy::new(|| TIM::new($data.maybe_lazy()))
    }};
}
