#![no_std]
#![feature(core_intrinsics)]
#![feature(alloc_error_handler)]
// These are not strictly necessary for writing a std library for the PSX, but they simplify things
#![feature(min_const_generics)]
#![feature(asm)]
#![feature(naked_functions)]

pub mod allocator;
pub mod bios;
pub mod registers;
pub mod gpu;
mod macros;

use core::intrinsics::volatile_load;
use core::panic::PanicInfo;

pub fn delay(n: u32) {
    for _ in 0..n {
        unsafe {
            volatile_load(0 as *mut u32);
        }
    }
}

#[macro_export]
macro_rules! exe {
    () => {
        use crate::executable::Ctxt;
        mod executable {
            use libpsx::gpu::{DisplayEnv, DrawEnv, GpuRead, GpuStat};
            pub struct Ctxt {
                draw_env: Option<DrawEnv>,
                display_env: Option<DisplayEnv>,
                gpu_read: Option<GpuRead>,
                gpu_stat: Option<GpuStat>,
            }

            impl Ctxt {
                pub fn take_draw_env(&mut self) -> Option<DrawEnv> {
                    self.draw_env.take()
                }

                pub fn take_display_env(&mut self) -> Option<DisplayEnv> {
                    self.display_env.take()
                }

                pub fn replace_draw_env(&mut self, draw_env: Option<DrawEnv>) {
                    self.draw_env = draw_env;
                }

                pub fn replace_display_env(&mut self, display_env: Option<DisplayEnv>) {
                    self.display_env = display_env;
                }
            }

            const ctxt: Ctxt = Ctxt {
                draw_env: Some(DrawEnv),
                display_env: Some(DisplayEnv),
                gpu_read: Some(GpuRead),
                gpu_stat: Some(GpuStat),
            };
            #[no_mangle]
            fn main() {
                super::main(ctxt)
            }
        }
    };
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if src < dest as *const u8 {
        // copy from end
        let mut i = n;
        while i != 0 {
            i -= 1;
            *dest.offset(i as isize) = *src.offset(i as isize);
        }
    } else {
        // copy from beginning
        let mut i = 0;
        while i < n {
            *dest.offset(i as isize) = *src.offset(i as isize);
            i += 1;
        }
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = c as u8;
        i += 1;
    }
    s
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.offset(i as isize);
        let b = *s2.offset(i as isize);
        if a != b {
            return a as i32 - b as i32;
        }
        i += 1;
    }
    0
}
