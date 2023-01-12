use crate::global::Global;
use core::mem::size_of;
use core::ptr::NonNull;

#[repr(C)]
struct GamepadBuffer {
    info: u16,
    buttons: u16,
    right_joystick: u16,
    left_joystick: u16,
    _padding: [u16; 13],
}

struct GamepadCtxt {
    buffer1: Option<NonNull<GamepadBuffer>>,
    buffer2: Option<NonNull<GamepadBuffer>>,
}

static CTXT: Global<GamepadCtxt> = Global::new(GamepadCtxt {
    buffer1: None,
    buffer2: None,
});

pub fn init_pad(buf1: &mut [u16], buf2: &mut [u16]) -> u32 {
    if buf1.len() != size_of::<GamepadBuffer>() {
        return 1
    }
    if buf2.len() != size_of::<GamepadBuffer>() {
        return 1
    }
    unsafe {
        CTXT.as_mut().buffer1 = NonNull::new(buf1).map(|p| p.cast());
        CTXT.as_mut().buffer2 = NonNull::new(buf2).map(|p| p.cast());
    }
    0
}
