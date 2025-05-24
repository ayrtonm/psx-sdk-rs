use critical_section::RawRestoreState;

use crate::sys::kernel;

/// Implements [`critical-section`](critical_section) using BIOS sys-calls.
struct BIOSCriticalSection;

critical_section::set_impl!(BIOSCriticalSection);

unsafe impl critical_section::Impl for BIOSCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        unsafe { kernel::psx_enter_critical_section() }
    }

    unsafe fn release(token: RawRestoreState) {
        if token {
            unsafe {
                kernel::psx_exit_critical_section();
            }
        };
    }
}
