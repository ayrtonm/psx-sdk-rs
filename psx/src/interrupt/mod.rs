//! Memory-mapped interrupt status and mask registers.

use crate::cop0;

mod irq;
pub mod mask;
pub mod stat;

pub(self) use irq::ALL_IRQS;
pub use irq::IRQ;

#[inline(always)]
pub fn free<F: FnOnce() -> R, R>(f: F) -> R {
    let stat = cop0::Status::read();
    if stat.interrupts_enabled() {
        stat.disable_interrupts().write();
        let ret = f();
        cop0::Status::read().enable_interrupts().write();
        ret
    } else {
        f()
    }
}
