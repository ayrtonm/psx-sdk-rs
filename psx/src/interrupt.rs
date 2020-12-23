use crate::cop0;

/// Executes the given closure in an interrupt-free context and returns the
/// result.
pub fn free<F: FnOnce() -> R, R>(f: F) -> R {
    let mut status = cop0::Status;
    let status = status.load_mut();
    if status.interrupts_enabled() {
        status.disable_interrupts().store();
        let ret = f();
        cop0::Status.load_mut().enable_interrupts().store();
        ret
    } else {
        f()
    }
}
