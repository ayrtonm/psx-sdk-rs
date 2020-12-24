#![allow(missing_docs)]
#![allow(warnings)]
use crate::bios;
use crate::cop0;
use crate::value::LoadMut;

/// Executes the given closure in a critical section and returns the result.
pub fn critical_section<F: FnOnce() -> R, R>(f: F) -> R {
    bios::enter_critical_section();
    //cop0::Status.load_mut().enter_critical_section().store();
    let r = f();
    bios::exit_critical_section();
    //cop0::Status.load_mut().exit_critical_section().store();
    r
}

/// Resets the GPU and installs a VSync event handler
pub fn reset_graph() {
    todo!("")
}

pub fn vsync() {
    todo!("")
}
