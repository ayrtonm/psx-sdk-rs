pub use crate::hal::irq::ty::IRQ;

pub fn free<F: FnOnce() -> R, R>(_f: F) -> R {
    todo!("");
}
