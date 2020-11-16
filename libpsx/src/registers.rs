use core::intrinsics::{volatile_load, volatile_store};
use core::ops::RangeInclusive;

pub trait BitTwiddle {
    fn set(&self, idx: u32) -> u32;
    fn clear(&self, idx: u32) -> u32;
    fn toggle(&self, idx: u32) -> u32;
    fn bit(&self, idx: u32) -> u32;
    fn bits(&self, idx: RangeInclusive<u32>) -> u32;
}

impl BitTwiddle for u32 {
    fn set(&self, idx: u32) -> u32 {
        *self | (1 << idx)
    }

    fn clear(&self, idx: u32) -> u32 {
        *self & !(1 << idx)
    }

    fn toggle(&self, idx: u32) -> u32 {
        *self ^ (1 << idx)
    }

    fn bit(&self, idx: u32) -> u32 {
        (*self >> idx) & 1
    }

    fn bits(&self, idx: RangeInclusive<u32>) -> u32 {
        let mask = (1 << idx.end()) - 1;
        (*self & mask) >> idx.start()
    }
}

pub trait Addr {
    const ADDRESS: u32;
}

pub trait Read: Addr {
    fn read(&self) -> u32 {
        unsafe { volatile_load(Self::ADDRESS as *const u32) }
    }
}

pub trait Write: Addr {
    fn write(&mut self, value: u32) {
        unsafe { volatile_store(Self::ADDRESS as *mut u32, value) }
    }

    fn write_slice(&mut self, values: &[u32]) {
        for v in values {
            self.write(*v)
        }
    }
}

pub trait Update: Read + Write {
    // TODO: add some debug checks to these functions
    fn update(&mut self, idx: u32, value: u32) {
        let current_value = self.read();
        let new_value = current_value.clear(idx) | (value << idx);
        self.write(new_value);
    }
    fn update_bits(&mut self, idx_range: RangeInclusive<u32>, value: u32) {
        let current_value = self.read();
        // For example update_bits(2..=4, x)
        // lower mask 0000...00011
        // upper mask 0000...01111
        // mask       0000...01100
        let lower_mask = (1 << idx_range.start()) - 1;
        let upper_mask = (1 << idx_range.end()) - 1;
        let mask = upper_mask ^ lower_mask;
        let new_value = current_value & !(mask << idx_range.start()) | (value << idx_range.start());
        self.write(new_value);
    }
}

impl<T: Read + Write> Update for T {}

#[macro_export]
macro_rules! ro_register {
    ($name:ident, $addr:expr) => {
        pub struct $name;
        impl crate::registers::Addr for $name {
            const ADDRESS: u32 = $addr;
        }
        impl crate::registers::Read for $name {}
    };
}

#[macro_export]
macro_rules! wo_register {
    ($name:ident, $addr:expr) => {
        pub struct $name;
        impl crate::registers::Addr for $name {
            const ADDRESS: u32 = $addr;
        }
        impl crate::registers::Write for $name {}
    };
}

#[macro_export]
macro_rules! rw_register {
    ($name:ident, $addr:expr) => {
        pub struct $name;
        impl crate::registers::Addr for $name {
            const ADDRESS: u32 = $addr;
        }
        impl crate::registers::Read for $name {}
        impl crate::registers::Write for $name {}
    };
}
