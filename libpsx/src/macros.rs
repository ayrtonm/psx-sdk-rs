#[macro_export]
macro_rules! register_def {
    ($name:ident) => {
        pub struct $name;
    }
}
#[macro_export]
macro_rules! register_addr {
    ($name:ident, $addr:expr) => {
        impl $name {
            const ADDRESS: u32 = $addr;
        }
    }
}

#[macro_export]
macro_rules! register_read {
    ($name:ident, $addr:expr) => {
        impl $name {
            pub fn read(&self) -> u32 {
                unsafe {
                    core::intrinsics::volatile_load($name::ADDRESS as *const u32)
                }
            }
        }
    }
}

#[macro_export]
macro_rules! register_write {
    ($name:ident, $addr:expr) => {
        impl $name {
            pub fn write(&mut self, value: u32) {
                unsafe {
                    core::intrinsics::volatile_store($name::ADDRESS as *mut u32, value)
                }
            }
            pub fn write_slice(&mut self, values: &[u32]) {
                for v in values {
                    self.write(*v)
                }
            }
        }
    }
}

#[macro_export]
macro_rules! ro_register {
    ($name:ident, $addr:expr) => {
        crate::register_def!($name);
        crate::register_addr!($name, $addr);
        crate::register_read!($name, $addr);
    }
}

#[macro_export]
macro_rules! wo_register {
    ($name:ident, $addr:expr) => {
        crate::register_def!($name);
        crate::register_addr!($name, $addr);
        crate::register_write!($name, $addr);
    }
}

#[macro_export]
macro_rules! rw_register {
    ($name:ident, $addr:expr) => {
        crate::register_def!($name);
        crate::register_addr!($name, $addr);
        crate::register_read!($name, $addr);
        crate::register_write!($name, $addr);
    }
}
