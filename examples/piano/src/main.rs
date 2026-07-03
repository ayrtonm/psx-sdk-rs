#![no_std]
#![no_main]

use alloc::string::String;
use psx::gpu::VideoMode;
use psx::hw::spu::adsr::{ADSRBuilder, EnvelopeDirection, EnvelopeMode};
use psx::hw::spu::volume::Volume;
use psx::hw::spu::Spu;
use psx::sys::event::{Event, Poll};
use psx::sys::gamepad::{Button, Gamepad};
use psx::{dprintln, println, Framebuffer};

psx::sys_heap!(500 KB);
#[unsafe(no_mangle)]
fn main() {
    let vblank_event = Event::<Poll>::new(0xF2000003, 0x0002).unwrap();

    let buf0 = (0, 0);
    let buf1 = (0, 240);
    let res = (320, 240);
    let txt_offset = (0, 8);
    let mut fb = Framebuffer::new(buf0, buf1, res, VideoMode::NTSC, None).unwrap();
    let font = fb.load_default_font();
    let mut txt = font.new_text_box(txt_offset, res);
    let mut gamepad = Gamepad::new();

    let mut spu = Spu::new();
    spu.reset();

    // Load piano sample into the SPU's RAM, from address 1024 (the end of the SPU
    // internal register area).
    let piano = include_bytes!("piano.adpcm");

    spu.write_cpu(1024, unsafe {
        core::mem::transmute::<&[u8], &[u16]>(piano)
    });

    // SPU DMA write doesn't seem to work
    //spu.write_dma(0).send_blocks_and(
    //    unsafe { core::mem::transmute::<&[u8], &[u32]>(piano) },
    //    16,
    //    || {},
    //);

    let mut channel = spu.channel(0).unwrap();

    let mut frequency = 1024;

    channel.sample_start(1024);
    channel.reverb(false);
    channel.volume(&Volume::Normal(0x3FFF));
    channel.frequency(frequency);
    channel.adsr(
        const {
            ADSRBuilder::new()
                .attack(EnvelopeMode::Exponential, 0, 0)
                .decay(0)
                .sustain(
                    0x0F,
                    EnvelopeMode::Linear,
                    EnvelopeDirection::Increase,
                    0,
                    0,
                )
                .release(EnvelopeMode::Linear, 0x7)
                .build()
        },
    );

    let mut keyon = false;
    let mut freq_change = false;
    loop {
        let buttons = gamepad.poll_p1();

        if buttons.pressed(Button::Cross) {
            if !keyon {
                channel.key_on();
                keyon = true;
            }
        } else {
            if keyon {
                channel.key_off();
                keyon = false;
            }
        }

        if buttons.pressed(Button::Up) {
            if !freq_change {
                frequency += 4;
                channel.frequency(frequency);
                freq_change = true;
            }
        } else {
            if freq_change {
                freq_change = false;
            }
        }

        if buttons.pressed(Button::Down) {
            if !freq_change {
                frequency -= 4;
                channel.frequency(frequency);
                freq_change = true;
            }
        } else {
            if freq_change {
                freq_change = false;
            }
        }

        dprintln!(
            txt,
            "Frequency: {:x} ({} Hz)",
            frequency,
            (44100u32 * frequency as u32) >> 12
        );
        dprintln!(txt, "Hold X to key on, and release to key off");
        txt.reset();
        fb.draw_sync();
        vblank_event.wait();
        fb.swap();
    }
}
