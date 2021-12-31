#![no_std]
#![no_main]

use framebuffer::constants::*;
use framebuffer::{draw_sync, enable_vblank, vsync, Framebuffer, Plane2, Plane3, Result, V2, V3};
use psx::gpu::colors::*;
use psx::gpu::primitives::PolyG4;
use psx::println;
use psx::sys::gamepad::{Button, Gamepad, BUFFER_SIZE};

psx::heap! {
    unsafe {
        psx::data_cache()
    }
}

const BUF0: V2 = ZERO2;
const BUF1: V2 = V2(0, 240);
const RES: V2 = V2(320, 240);

fn basic(v: V3) -> V2 {
    let ex = (v.2 as f32 / 2.0) as i16;
    let ey = (v.2 as f32 / 2.0) as i16;
    V2(v.0 + ex, v.1 + ey)
}

fn sum_z(plane: &Plane3) -> i16 {
    let mut res = 0;
    for V3(_x, _y, z) in *plane {
        res += z;
    }
    -res
}

#[no_mangle]
fn main() -> Result<()> {
    let mut fb = Framebuffer::new(BUF0, BUF1, RES)?;
    let mut buf0 = [0; BUFFER_SIZE];
    let mut buf1 = [0; BUFFER_SIZE];
    let mut pad = Gamepad::new(&mut buf0, &mut buf1)?;

    let mut cube = [XY, YZ, XZ, XY + Z, YZ + X, XZ + Y].map(|p| p * 50);
    cube.sort_by_key(sum_z);
    println!("{:?}", cube);
    let mut quads = [PolyG4::new(); 6];
    for q in &mut quads {
        q.set_colors([INDIGO, ORANGE, MINT, YELLOW]);
    }
    enable_vblank();
    loop {
        let mut shift = ZERO;
        if pad.pressed(Button::Up) {
            shift -= Z;
        }
        if pad.pressed(Button::Down) {
            shift += Z;
        }
        if pad.pressed(Button::Left) {
            shift -= X;
        }
        if pad.pressed(Button::Right) {
            shift += X;
        }
        for p in cube {
            p += shift;
        }
        if pad.pressed(Button::Cross) {
            println!("pressed X");
        }
        for (n, q) in quads.iter_mut().enumerate() {
            let plane = cube[n];
            q.set_vertices(plane.project(basic).into());
            fb.gp0.send_command(q);
            draw_sync();
        }
        vsync();
        fb.swap(None)?;
    }
}
