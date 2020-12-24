#![allow(missing_docs)]
#![allow(warnings)]
use core::ffi::c_void;

use crate::bios;
use crate::cop0;
use crate::dma;
use crate::dma::{gpu, otc, Channel};
use crate::value::LoadMut;

/// Executes the given closure in a critical section and returns the result.
#[no_mangle]
pub fn critical_section<F: FnOnce() -> R, R>(f: F) -> R {
    bios::enter_critical_section();
    //cop0::Status.load_mut().enter_critical_section().store();
    let r = f();
    bios::exit_critical_section();
    //cop0::Status.load_mut().exit_critical_section().store();
    r
}

const reset_graph_msg: &'static [u8] = b"hello world\0";
const sr_msg: &'static [u8] = b"replace me\0";

/// Resets the GPU and installs a VSync event handler
#[no_mangle]
pub fn reset_graph() {
    bios::printf(reset_graph_msg.as_ptr(), 0xdead_beef);
    bios::printf(sr_msg.as_ptr(), 0xdead_beef);
    critical_section(|| {
        dma::control::Control
            .skip_load()
            .disable_all()
            .enable(Channel::GPU)
            .enable(Channel::OTC)
            .store();
        dma::interrupt::Interrupt
            .skip_load()
            .clear(0xFFFF_FFFF)
            .store();
        interrupt_callback();
        restart_callback();
        bios::cd_remove();
    });
}

#[no_mangle]
pub fn interrupt_callback() {}

#[no_mangle]
pub fn restart_callback() {}

#[no_mangle]
pub fn vsync() {
    todo!("")
}
