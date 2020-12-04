use core::ptr::{read_volatile, write_volatile};

pub trait Address: Sized {
    const ADDRESS: u32;
}

pub trait Read: Address {
    unsafe fn read(&self) -> u32 {
        read_volatile(Self::ADDRESS as *const u32)
    }
}

pub trait Write: Address {
    unsafe fn write(&mut self, value: u32) {
        write_volatile(Self::ADDRESS as *mut u32, value)
    }
}

pub trait Update: Read + Write {
    unsafe fn update<F>(&mut self, f: F)
    where F: FnOnce(u32) -> u32 {
        let current_value = self.read();
        let new_value = f(current_value);
        self.write(new_value);
    }
}
