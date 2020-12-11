//! Memory-mapped interrupt status and mask registers.

use crate::cop0;

mod irq;
mod mask;
mod stat;

pub use irq::IRQ;
pub(self) use irq::ALL_IRQS;

pub fn disable() {
    let mut status = cop0::Status::read();
    status.remove(cop0::Status::IEC);
    status.write();
}

pub fn enable() {
    let mut status = cop0::Status::read();
    status.insert(cop0::Status::IEC);
    status.write();
}

pub fn enabled() -> bool {
    let status = cop0::Status::read();
    status.contains(cop0::Status::IEC)
}

pub fn free<F: FnOnce() -> R, R>(f: F) -> R {
    if enabled() {
        disable();
        let ret = f();
        enable();
        ret
    } else {
        f()
    }
}
