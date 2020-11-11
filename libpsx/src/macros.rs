#[macro_export]
macro_rules! ro_register {
    ($name:ident, $addr:expr) => {
        impl $name {
            const ADDRESS: u32 = $addr;
            pub fn read(&self) -> u32 {
                unsafe {
                    volatile_load($name::ADDRESS as *const u32)
                }
            }
        }
    }
}

#[macro_export]
macro_rules! rw_register {
    ($name:ident, $addr:expr) => {
        crate::ro_register!($name, $addr);
        impl $name {
            pub fn write(&mut self, value: u32) {
                unsafe {
                    volatile_store($name::ADDRESS as *mut u32, value)
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

