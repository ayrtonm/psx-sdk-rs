#![no_std]
#![no_main]

use alloc::string::String;
use psx::gpu::VideoMode;
use psx::sys::event::{Event, Poll};
use psx::sys::gamepad::Gamepad;
use psx::{Framebuffer, dprintln, println};

psx::sys_heap!(500 KB);
#[unsafe(no_mangle)]
fn main() {
    // We don't use Framebuffer's functiion to wait for VBlank, as that relies on
    // the raw IRQ register (which is impossible to use alongside the BIOS'
    // gamepad handler without taking over the kernel). So we register a polling
    // BIOS event on the VBlank IRQ.
    let vblank_event = Event::<Poll>::new(0xF2000003, 0x0002).unwrap();

    let buf0 = (0, 0);
    let buf1 = (0, 240);
    let res = (320, 240);
    let txt_offset = (0, 8);
    let mut fb = Framebuffer::new(buf0, buf1, res, VideoMode::NTSC, None).unwrap();
    let font = fb.load_default_font();
    let mut txt = font.new_text_box(txt_offset, res);

    // If we didn't have Gamepad here. we'd have to change the BIOS event to be a
    // callback instead of a polling event (because the BIOS doesn't
    // automatically acknowledge the IRQ at the end of the chain and requires
    // one of the chain events to do so instead)
    //
    // Gamepad happens to auto-acknowledge VBlank IRQs, if change_clear_pad is set
    // to 1
    let mut gamepad = Gamepad::new();
    loop {
        let mut but_str = String::new();

        for button in gamepad.poll_p1() {
            but_str += match button {
                psx::sys::gamepad::Button::Up => " U ",
                psx::sys::gamepad::Button::Down => " D ",
                psx::sys::gamepad::Button::Left => " L ",
                psx::sys::gamepad::Button::Right => " R ",
                psx::sys::gamepad::Button::Triangle => " /\\ ",
                psx::sys::gamepad::Button::Cross => " X ",
                psx::sys::gamepad::Button::Circle => " O ",
                psx::sys::gamepad::Button::Square => " [] ",
                psx::sys::gamepad::Button::L1 => " L1 ",
                psx::sys::gamepad::Button::R1 => " R1 ",
                psx::sys::gamepad::Button::L2 => " L2 ",
                psx::sys::gamepad::Button::R2 => " R2 ",
                psx::sys::gamepad::Button::L3 => " L3 ",
                psx::sys::gamepad::Button::R3 => " R3 ",
                psx::sys::gamepad::Button::Start => " S ",
                psx::sys::gamepad::Button::Select => " s ",
            };
        }

        txt.reset();
        dprintln!(txt, "Buttons: {}", but_str);
        fb.draw_sync();
        vblank_event.wait();
        fb.swap();
    }
}
