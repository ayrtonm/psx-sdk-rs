use core::ptr::{read_volatile, write_volatile};

// These traits exist in public interfaces, but the module is pub(cate) so they
// can't be used or implemented outside the crate since that would require
// importing this module.
pub trait Address: Sized {
    const ADDRESS: u32;
}

pub trait Read<T>: Address {
    #[inline(always)]
    unsafe fn read(&self) -> T {
        read_volatile(Self::ADDRESS as *const T)
    }
}

pub trait Write<T>: Address {
    #[inline(always)]
    unsafe fn write(&mut self, value: T) {
        write_volatile(Self::ADDRESS as *mut T, value)
    }
}

//#[deprecated]
pub trait Update<T>: Read<T> + Write<T>
where u32: From<T> {
    unsafe fn update<F>(&mut self, f: F)
    where F: FnOnce(T) -> T {
        let current_value = self.read();
        let new_value = f(current_value);
        self.write(new_value);
    }
}
