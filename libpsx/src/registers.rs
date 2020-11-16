pub trait BitTwiddle {
    fn set(&self, idx: usize) -> u32;
    fn clear(&self, idx: usize) -> u32;
    fn toggle(&self, idx: usize) -> u32;
}

impl BitTwiddle for u32 {
    fn set(&self, idx: usize) -> u32 {
        *self | (1 << idx)
    }
    fn clear(&self, idx: usize) -> u32 {
        *self & !(1 << idx)
    }
    fn toggle(&self, idx: usize) -> u32 {
        *self ^ (1 << idx)
    }
}

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
macro_rules! ro_register {
    ($name:ident, $addr:expr) => {
        pub struct $name;
        impl crate::registers::RegisterAddr for $name { const ADDRESS: u32 = $addr; }
        impl crate::registers::RegisterRead for $name {}
    };
}

#[macro_export]
macro_rules! wo_register {
    ($name:ident, $addr:expr) => {
        pub struct $name;
        impl crate::registers::RegisterAddr for $name { const ADDRESS: u32 = $addr; }
        impl crate::registers::RegisterWrite for $name {}
    };
}

#[macro_export]
macro_rules! rw_register {
    ($name:ident, $addr:expr) => {
        pub struct $name;
        impl crate::registers::RegisterAddr for $name { const ADDRESS: u32 = $addr; }
        impl crate::registers::RegisterRead for $name {}
        impl crate::registers::RegisterWrite for $name {}
    };
}
