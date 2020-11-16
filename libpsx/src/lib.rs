#![no_std]
#![feature(core_intrinsics)]
#![feature(alloc_error_handler)]
// These are not strictly necessary for writing a std library for the PSX, but they simplify things
#![feature(min_const_generics)]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(doc_cfg)]

pub mod allocator;
pub mod bios;
mod builtins;
pub mod dma;
pub mod gpu;
mod macros;

#[cfg(doc)]
mod docs;
#[cfg(doc)]
pub use crate::docs::executable::{main, Ctxt};

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
    (big heap) => {
        libpsx::exe!(0x1F00_0000, 8192 * 1024);
    };
    (fast heap) => {
        libpsx::exe!(0x1F80_0000, 1024);
    };
    () => {
        libpsx::exe!(0x1FA0_0000, 2048 * 1024);
    };
    (no heap) => {
        libpsx::exe!(0, 0);
    };
    ($heap_addr:expr, $heap_size:expr) => {
        #[cfg(not(doc))]
        use crate::executable::Ctxt;

        // TODO: fix the linker error with rust-lld
        extern crate alloc;
        // TODO: add other common collections here
        use alloc::borrow::Cow;
        use alloc::boxed::Box;
        use alloc::rc::Rc;
        use alloc::vec::Vec;

        pub mod executable {

            #[cfg(doc)]
            use crate::gpu::{DispPort, DrawPort, GpuRead, GpuStat};
            #[cfg(not(doc))]
            use libpsx::gpu::{DispPort, DrawPort, GpuRead, GpuStat};

            pub struct Ctxt {
                draw_port: Option<DrawPort>,
                disp_port: Option<DispPort>,
                gpu_read: Option<GpuRead>,
                gpu_stat: Option<GpuStat>,
            }

            impl Ctxt {
                pub fn take_draw_port(&mut self) -> Option<DrawPort> {
                    self.draw_port.take()
                }

                pub fn take_disp_port(&mut self) -> Option<DispPort> {
                    self.disp_port.take()
                }

                pub fn replace_draw_port(&mut self, draw_port: Option<DrawPort>) {
                    self.draw_port = draw_port;
                }

                pub fn replace_disp_port(&mut self, disp_port: Option<DispPort>) {
                    self.disp_port = disp_port;
                }
            }

            #[cfg(not(doc))]
            const ctxt: Ctxt = Ctxt {
                draw_port: Some(DrawPort),
                disp_port: Some(DispPort),
                gpu_read: Some(GpuRead),
                gpu_stat: Some(GpuStat),
            };

            //TODO: remove link_section (or change to .text) for regular .psexe's
            #[cfg(not(doc))]
            #[no_mangle]
            //#[link_section = ".exe"]
            fn main() {
                #[cfg(not(doc))]
                if $heap_size != 0 {
                    libpsx::bios::init_heap($heap_addr, $heap_size);
                }
                super::main(ctxt)
            }

            #[cfg(doc)]
            pub fn main(mut ctxt: Ctxt) {}
        }
    };
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
