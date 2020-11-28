#![macro_use]

macro_rules! register {
    ($name:ident, $address:expr) => {
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
    ($name:ident, $address:expr) => {
        register!($name, $address);
        impl $crate::mmio::register::Read for $name {}
    };
}

macro_rules! write_only {
    ($name:ident, $address:expr) => {
        register!($name, $address);
        impl $crate::mmio::register::Write for $name {}
    };
}

macro_rules! read_write {
    ($name:ident, $address:expr) => {
        register!($name, $address);
        impl $crate::mmio::register::Read for $name {}
        impl $crate::mmio::register::Write for $name {}
        impl $crate::mmio::register::Update for $name {}
    };
}
