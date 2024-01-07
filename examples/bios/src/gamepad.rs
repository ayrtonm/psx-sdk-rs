use crate::global::Global;
use crate::println;
use crate::thread;
use crate::thread::{ThreadControlBlock, Thread};
use crate::exceptions::{IRQCtxt, enqueue_handler};
use core::mem::size_of;
use core::ptr;
use core::ptr::NonNull;
use psx::hw::pad::{BaudFactor, CharLength, RxIntMode};
use psx::hw::{cop0, pad, Register};
use psx::hw::irq::IRQ;

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
        buffer2: NonNull::new(buf2).unwrap().cast(),
    };

    pad::Baud::skip_load().set_bits(0x88).store();
    pad::Control::skip_load()
        .enable_tx()
        .enable_output()
        .enable_rx_interrupt()
        .enable_ack_interrupt()
        .set_rx_interrupt_mode(RxIntMode::Bits1)
        .store();
    pad::Mode::skip_load()
        .set_baud_factor(BaudFactor::Mul1)
        .set_char_length(CharLength::Bits8)
        .store();
    cop0::Status::new().critical_section(|cs| {
        enqueue_handler(poll_gamepad, cs);
    });
    pad::TxData::send(0x01);
    0
}

fn poll_gamepad(ctxt: IRQCtxt) -> *mut ThreadControlBlock {
    const SEQ: [u8; 5] = [1, 0x42, 0, 0, 0];
    static N: Global<usize> = Global::new(1);
    let n = N.borrow(ctxt.cs);
    if ctxt.mask.get_requested(ctxt.stat).contains(IRQ::ControllerMemoryCard) && *n < SEQ.len() {
        let mut ctrl = pad::Control::new();
        ctrl.disable_output().store();
        ctrl.enable_output().store();

        println!("Received {:x?} and sending {:x?}", pad::RxData::recv(), SEQ[*n]);
        pad::TxData::send(SEQ[*n]);
        *n += 1;

        ctxt.stat.ack(IRQ::ControllerMemoryCard).store();
        pad::Control::new().ack().store();
    } else if ctxt.mask.get_requested(ctxt.stat).contains(IRQ::Vblank) && *n == SEQ.len() {
        *n = 0;
        pad::TxData::send(0x01);
    }

    //    *n += 1;
    //    if *n < SEQ.len() {
    //        //*n = 0;
    //        let mut ctrl = pad::Control::new();
    //        ctrl.disable_output().store();
    //        ctrl.enable_output().store();
    //    //}
    //    println!("Received {:x?} and sending {:x?}", pad::RxData::recv(), SEQ[*n]);
    //    pad::TxData::send(SEQ[*n]);

    //    ctxt.stat.ack(IRQ::ControllerMemoryCard).store();
    //    pad::Control::new().ack().store();
    //    }
    ptr::null_mut()
}
