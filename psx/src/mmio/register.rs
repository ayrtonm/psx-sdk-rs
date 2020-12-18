use core::ptr::{read_volatile, write_volatile};

// These traits exist in public interfaces, but the module is pub(cate) so they
// can't be used or implemented outside the crate since that would require
// importing this module.
pub trait Address<T: Into<u32>>: Sized {
    const ADDRESS: T;
}

pub trait Read<T: Into<u32>>: Address<T> {
    unsafe fn read(&self) -> T {
        read_volatile(Self::ADDRESS.into() as *const T)
    }
}

pub trait Write<T: Into<u32>>: Address<T> {
    unsafe fn write(&mut self, value: T) {
        write_volatile(Self::ADDRESS.into() as *mut T, value)
    }
}

pub trait Update<T: Into<u32>>: Read<T> + Write<T> {
    unsafe fn update<F>(&mut self, f: F)
    where F: FnOnce(T) -> T {
        let current_value = self.read();
        let new_value = f(current_value);
        self.write(new_value);
    }
}
