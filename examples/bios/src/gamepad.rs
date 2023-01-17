use crate::println;
use crate::thread;
use crate::thread::Thread;
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

unsafe impl Send for GamepadCtxt {}

#[repr(C)]
pub struct GamepadCtxt {
    buffer1: NonNull<GamepadBuffer>,
    buffer2: NonNull<GamepadBuffer>,
}

pub fn init() {
    static mut BUFFER1: [u16; 17] = [0; 17];
    static mut BUFFER2: [u16; 17] = [0; 17];
    unsafe {
        init_pad(BUFFER1.as_mut_slice(), BUFFER2.as_mut_slice());
    }
}

pub fn init_pad(buf1: &mut [u16], buf2: &mut [u16]) -> u32 {
    if buf1.len() != size_of::<GamepadBuffer>() / size_of::<u16>() {
        return 1
    };
    if buf2.len() != size_of::<GamepadBuffer>() / size_of::<u16>() {
        return 1
    };
    let ctxt = GamepadCtxt {
        buffer1: NonNull::new(buf1).unwrap().cast(),
        buffer2: NonNull::new(buf1).unwrap().cast(),
    };
    let mut t = Thread::create_with_arg(gamepad_thread, ctxt).unwrap();
    t.unpark();
    0
}

extern "C" fn gamepad_thread(_ctxt: GamepadCtxt) {
    println!("Started gamepad thread");
    loop {
        thread::resume_main();
    }
}
