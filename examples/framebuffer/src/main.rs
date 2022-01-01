#![no_std]
#![no_main]

use libm::{acosf, cosf, sinf};
use psx::dma;
use psx::gpu::colors::*;
use psx::gpu::primitives::*;
use psx::gpu::Packet;
use psx::println;
use psx::sys::rng::Rng;

use framebuffer::constants::{XY2, ZERO2};
use framebuffer::{draw_sync, enable_vblank, vsync, Framebuffer, Result, V2};

const BUF0: V2 = V2(0, 0);
const BUF1: V2 = V2(0, 240);
const RES: V2 = V2(320, 240);

fn rand_vector(rng: &mut Rng) -> V2 {
    V2(rng.rand(), rng.rand())
}

#[no_mangle]
fn main() -> Result<()> {
    let mut gpu_dma = dma::GPU::new();
    let mut fb = Framebuffer::new(BUF0, BUF1, RES)?;
    let mut rng = Rng::new(0x12349876);
    let mut quad = Packet::new(PolyG4::new())?;

    let pi = acosf(-1.0);

    let mut theta = 0.0;
    let mut dir = 1.0;

    let len = 50;
    let vertices = (XY2 * len) - len / 2;

    let cx = rng.rand::<u16>() % 320;
    let cy = rng.rand::<u16>() % 240;
    let mut center = V2(cx as i16, cy as i16);
    let mut velocity = rand_vector(&mut rng) % 5;

    quad.payload.set_colors([INDIGO, ORANGE, MINT, YELLOW]);
    enable_vblank();
    loop {
        theta += dir * pi / 100.0;
        if theta >= 2.0 * pi {
            theta -= 2.0 * pi;
        }
        center += velocity;

        let new_vertices = vertices.R(theta, ZERO2) + center;

        for V2(x, y) in new_vertices {
            if x >= 320 || x <= 0 {
                velocity.0 *= -1;
                dir *= -1.0;
                break
            }
            if y >= 240 || y <= 0 {
                velocity.1 *= -1;
                dir *= -1.0;
                break
            }
        }
        draw_sync();
        quad.payload.set_vertices(new_vertices.into());
        vsync();
        fb.swap(Some(&mut gpu_dma))?;

        gpu_dma.send_list(&quad)?;
    }
}
