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

        impl $crate::mmio::register::Address<u32> for $name {
            const ADDRESS: u32 = $address;
        }
    };
}

macro_rules! read_only {
    ($(#[$meta:meta])* $name:ident, $address:expr) => {
        register!($(#[$meta])* $name, $address);
        impl $crate::mmio::register::Read<u32> for $name {}
    };
}

macro_rules! write_only {
    ($(#[$meta:meta])* $name:ident, $address:expr) => {
        register!($(#[$meta])* $name, $address);
        impl $crate::mmio::register::Write<u32> for $name {}
    };
}

macro_rules! read_write {
    ($(#[$meta:meta])* $name:ident, $address:expr) => {
        register!($(#[$meta])* $name, $address);
        impl $crate::mmio::register::Read<u32> for $name {}
        impl $crate::mmio::register::Write<u32> for $name {}
        impl $crate::mmio::register::Update<u32> for $name {}
    };
}
