pub mod kernel;

/// Prints a formatted message with up to eight arguments to the TTY console.
#[macro_export]
macro_rules! printf {
    ($msg:expr $(,$args:expr)*) => {
        unsafe {
            $crate::bios::kernel::printf($msg.as_ptr() $(,$args)*)
        }
    };
}

pub fn malloc(size: usize) -> *mut u8 {
    unsafe { kernel::malloc(size) }
}

pub fn free(buf: *mut u8) {
    unsafe { kernel::free(buf) }
}

pub fn gpu_get_status() -> u32 {
    unsafe { kernel::gpu_get_status() }
}

pub fn init_pad(buf1: &mut [u8], buf2: &mut [u8]) {
    unsafe { kernel::init_pad(buf1.as_mut_ptr(), buf1.len(), buf2.as_mut_ptr(), buf2.len()) }
}
