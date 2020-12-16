//! Memory-mapped interrupt status and mask registers.

// TODO: This generates really bad code. It'd probably be best to handwrite
// common idioms in assembly instead of making these small functions.

use crate::cop0;

mod irq;
mod mask;
mod stat;

pub(self) use irq::ALL_IRQS;
pub use irq::IRQ;

#[inline(always)]
pub fn disable() {
    let mut status = cop0::Status::read();
    status.remove(cop0::Status::IEC);
    status.write();
}

#[inline(always)]
pub fn enable() {
    let mut status = cop0::Status::read();
    status.insert(cop0::Status::IEC);
    status.write();
}

#[inline(always)]
pub fn enabled() -> bool {
    let status = cop0::Status::read();
    status.contains(cop0::Status::IEC)
}

#[inline(always)]
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
