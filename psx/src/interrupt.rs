pub use crate::hal::irq::ty::IRQ;

pub fn free<F: FnOnce() -> R, R>(_f: F) -> R {
    todo!(
        "
        if ints_enabled {{
            disable_ints
            let res = f();
            enable_ints
            res
        }} else {{
            f()
        }}
        "
    )
}
