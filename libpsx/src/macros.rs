pub trait RegisterAddr {
    const ADDRESS: u32;
}
pub trait RegisterRead: RegisterAddr {
    fn read(&self) -> u32 {
        unsafe { core::intrinsics::volatile_load(Self::ADDRESS as *const u32) }
    }
}
pub trait RegisterWrite: RegisterAddr {
    fn write(&mut self, value: u32) {
        unsafe { core::intrinsics::volatile_store(Self::ADDRESS as *mut u32, value) }
    }

    fn write_slice(&mut self, values: &[u32]) {
        for v in values {
            self.write(*v)
        }
    }
}
#[macro_export]
macro_rules! register_def {
    ($name:ident) => {
        pub struct $name;
    };
}
#[macro_export]
macro_rules! register_addr {
    ($name:ident, $addr:expr) => {
        impl crate::macros::RegisterAddr for $name {
            const ADDRESS: u32 = $addr;
        }
    };
}

#[macro_export]
macro_rules! register_read {
    ($name:ident) => {
        impl crate::macros::RegisterRead for $name {}
    };
}

#[macro_export]
macro_rules! register_write {
    ($name:ident) => {
        impl crate::macros::RegisterWrite for $name {}
    };
}

#[macro_export]
macro_rules! ro_register {
    ($name:ident, $addr:expr) => {
        crate::register_def!($name);
        crate::register_addr!($name, $addr);
        crate::register_read!($name);
    };
}

#[macro_export]
macro_rules! wo_register {
    ($name:ident, $addr:expr) => {
        crate::register_def!($name);
        crate::register_addr!($name, $addr);
        crate::register_write!($name);
    };
}

#[macro_export]
macro_rules! rw_register {
    ($name:ident, $addr:expr) => {
        crate::register_def!($name);
        crate::register_addr!($name, $addr);
        crate::register_read!($name);
        crate::register_write!($name);
    };
}
