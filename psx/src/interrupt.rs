use crate::hal::cop0::{IntMask, Status};
use crate::hal::{MutRegister, Register};

pub use crate::hal::irq::ty::IRQ;

pub fn critical_section<F: FnOnce() -> R, R>(f: F) -> R {
    let mut status = Status::load();
    if status.ints_enabled() {
        status
            .enable_ints(false)
            .mask_int(IntMask::Hardware, false)
            .store();
        let res = f();
        status
            .enable_ints(true)
            .mask_int(IntMask::Hardware, true)
            .store();
        res
    } else {
        f()
    }
}
