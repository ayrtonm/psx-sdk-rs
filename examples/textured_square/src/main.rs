#![no_std]
#![no_main]
#![feature(core_intrinsics)]

use core::cell::RefCell;
use libpsx::gpu::vertex::Vertex;
use libpsx::gpu::color::Color;
use libpsx::gpu::framebuffer::Framebuffer;
use libpsx::gpu::res::{Hres, Vres};

libpsx::exe!();

fn main(mut ctxt: Ctxt) {
    let draw_env = RefCell::new(ctxt.take_draw_env().expect("DrawEnv has been taken"));
    let display_env = RefCell::new(ctxt.take_display_env().expect("DisplayEnv has been taken"));
    let buf0 = (0, 0);
    let buf1 = (0, 240);
    let res = (Hres::H320, Vres::V240);
    display_env.borrow_mut().reset_gpu();
    let mut fb = Framebuffer::new(&draw_env, &display_env, buf0, buf1, res);
    const N: usize = 4;
    let mut data = [0; N];
    for i in 0..N {
        let addr = 0xBFC6_4000 + (i << 2);
        unsafe {
            data[i] = core::intrinsics::volatile_load(addr as *const u32);
        }
    }
    draw_env.borrow_mut().rect_to_vram((320, 0), (320, 240), &data);
    loop {
         //draw_env.borrow_mut().draw_rect(&Vertex::zero(), 320, 240, &Color::blue());
         draw_env.borrow_mut().draw_rect_textured(&Vertex::zero(), 160, 120, 0x7F0F_0005);
         fb.swap();
    }
}
