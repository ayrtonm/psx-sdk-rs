#![macro_use]

macro_rules! register {
    ($(#[$meta:meta])* $name:ident, $address:expr) => {
        $(#[$meta])*
        pub struct $name(());

        impl $name {
            pub(crate) unsafe fn new() -> Self {
                $name(())
            }
        }

        impl $crate::mmio::register::Address for $name {
            const ADDRESS: u32 = $address;
        }
    };
}

macro_rules! read_only {
    ($(#[$meta:meta])* $name:ident, $address:expr) => {
        register!($(#[$meta])* $name, $address);
        impl $crate::mmio::register::Read for $name {}
    };
}

macro_rules! write_only {
    ($(#[$meta:meta])* $name:ident, $address:expr) => {
        register!($(#[$meta])* $name, $address);
        impl $crate::mmio::register::Write for $name {}
    };
}

macro_rules! read_write {
    ($(#[$meta:meta])* $name:ident, $address:expr) => {
        register!($(#[$meta])* $name, $address);
        impl $crate::mmio::register::Read for $name {}
        impl $crate::mmio::register::Write for $name {}
        impl $crate::mmio::register::Update for $name {}
    };
}
