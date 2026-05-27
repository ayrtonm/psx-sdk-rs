//! Module for BIOS events.

use core::marker::PhantomData;

use crate::sys::{critical_section, kernel};

#[repr(C)]
struct InternalEvent {
    class: u32,
    status: u32,
    spec: u32,
    mode: u32,
    func_ptr: extern "C" fn(),
    _unused: [u8; 8],
}

const SYS_EVCB: *const u32 = 0x120 as _;
const SYS_EVCB_SIZE: *const u32 = 0x124 as _;

/// Get a pointer to the Event Control Block belonging to a event handler. This
/// pointer is within the first 64 kilobytes of the system RAM.
///
/// SAFETY: You have to check that the handle is within the range of the EvCB
/// table (by checking the size of the table against the handle), Otherwise, the
/// resulting pointer may be invalid.
unsafe fn get_event_ptr(event: u32) -> *mut InternalEvent {
    // SAFETY: This global variable should exist in all the Sony BIOSes.
    let evcb_ptr = unsafe { SYS_EVCB.read() } as *mut InternalEvent;

    return evcb_ptr.wrapping_add(event as usize & 0xFFFF);
}

/// Status of the event.
#[derive(Debug, PartialEq)]
pub enum EventStatus {
    /// The event is free/unallocated
    Unallocated,
    /// The event is disabled
    Disabled,
    /// The event is enabled.
    Enabled,
    /// The event is enabled and ready.
    Ready,
    /// Unknown status
    Unknown,
}

// Event modes
trait EventMode {
    const MODE_ID: u16;
}

/// An event in callback mode. When the event gets triggered, the BIOS will
/// execute the associated callback function.
pub struct Callback;

/// An event in polling mode. When the event gets triggered, the BIOS will set
/// the ready flag on the event.
pub struct Poll;

impl EventMode for Callback {
    const MODE_ID: u16 = 0x1000;
}
impl EventMode for Poll {
    const MODE_ID: u16 = 0x2000;
}

/// An event handle
pub struct Event<MODE: EventMode> {
    handle: u32,
    _mode: PhantomData<MODE>,
}

impl<MODE: EventMode> Event<MODE> {
    /// Get the status of the event.
    pub fn status(&self) -> EventStatus {
        // Since the kernel does not provide a function to get the status of the
        // event, we have to read the kernel's global variables to get it.

        // SAFETY: Once the event has been initialized by the BIOS, we can read the mode
        // field without synchronization.
        let evcb = unsafe { get_event_ptr(self.handle) };

        match unsafe { (*evcb).mode } {
            0 => EventStatus::Unallocated,
            0x1000 => EventStatus::Disabled,
            0x2000 => EventStatus::Enabled,
            0x4000 => EventStatus::Ready,
            _ => EventStatus::Unknown,
        }
    }
}

impl Event<Poll> {
    /// Create a new polling event.
    ///
    /// Returns an error if the handle it gets from the BIOS ends up being
    /// invalid.
    pub fn new(class: u32, spec: u16) -> Result<Event<Poll>, ()> {
        unsafe {
            critical_section(|_| {
                // The BIOS should never execute the callback, so we can set it to a null
                // pointer
                let handle = kernel::psx_open_event(class, spec, Poll::MODE_ID, 0x0 as _);

                // Check if the handle is outside the EvCB table, and return an error if it is,
                // since it most likely means an error occurred in the BIOS.
                if (handle & 0xFFFF) * 0x1C > SYS_EVCB_SIZE.read() {
                    return Err(());
                }

                kernel::psx_enable_event(handle);

                Ok(Event {
                    handle,
                    _mode: PhantomData::<Poll>,
                })
            })
        }
    }

    /// Test if the event has been triggered by the kernel. If it has, then
    /// it'll return `true`
    pub fn test(&self) -> bool {
        unsafe { kernel::psx_test_event(self.handle) }
    }

    /// Spin until the event becomes ready.
    ///
    /// NOTE: This will return immediately if the event is disabled. Thus, you
    /// should check the status of the event (with `.status()`) to see if
    /// it's disabled beforehand.
    pub fn wait(&self) {
        unsafe {
            kernel::psx_wait_event(self.handle);
        }
    }
}

impl Event<Callback> {
    /// Create a new callback event.
    ///
    /// Returns an error if the handle it gets from the BIOS ends up being
    /// invalid.
    pub fn new(class: u32, spec: u16, callback: extern "C" fn()) -> Result<Event<Callback>, ()> {
        unsafe {
            critical_section(|_| {
                let handle = kernel::psx_open_event(class, spec, Callback::MODE_ID, callback as _);

                // Check if the handle is outside the EvCB table, and return an error if it is,
                // since it most likely means an error occurred in the BIOS.
                if (handle & 0xFFFF) * 0x1C > SYS_EVCB_SIZE.read() {
                    return Err(());
                }

                kernel::psx_enable_event(handle);

                Ok(Event {
                    handle,
                    _mode: PhantomData::<Callback>,
                })
            })
        }
    }
}

impl<MODE: EventMode> Drop for Event<MODE> {
    fn drop(&mut self) {
        unsafe {
            kernel::psx_close_event(self.handle);
        }
    }
}
