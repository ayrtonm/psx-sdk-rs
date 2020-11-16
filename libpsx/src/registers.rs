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

pub trait Addr {
    const ADDRESS: u32;
}

pub trait Read: Addr {
    fn read(&self) -> u32 {
        unsafe { core::intrinsics::volatile_load(Self::ADDRESS as *const u32) }
    }
}

pub trait Write: Addr {
    fn write(&mut self, value: u32) {
        unsafe { core::intrinsics::volatile_store(Self::ADDRESS as *mut u32, value) }
    }

    fn write_slice(&mut self, values: &[u32]) {
        for v in values {
            self.write(*v)
        }
    }
}

pub trait Update: Read + Write {
    fn update(&mut self, idx: usize, value: u32) {
        // TODO: add some debug checks here
        let current_value = self.read();
        let new_value = current_value.clear(idx) | (value << idx);
        self.write(new_value);
    }
}

impl<T: Read + Write> Update for T {}

#[macro_export]
macro_rules! ro_register {
    ($name:ident, $addr:expr) => {
        pub struct $name;
        impl crate::registers::Addr for $name { const ADDRESS: u32 = $addr; }
        impl crate::registers::Read for $name {}
    };
}

#[macro_export]
macro_rules! wo_register {
    ($name:ident, $addr:expr) => {
        pub struct $name;
        impl crate::registers::Addr for $name { const ADDRESS: u32 = $addr; }
        impl crate::registers::Write for $name {}
    };
}

#[macro_export]
macro_rules! rw_register {
    ($name:ident, $addr:expr) => {
        pub struct $name;
        impl crate::registers::Addr for $name { const ADDRESS: u32 = $addr; }
        impl crate::registers::Read for $name {}
        impl crate::registers::Write for $name {}
    };
}
