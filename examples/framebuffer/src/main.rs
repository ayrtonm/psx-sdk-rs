#![no_std]
#![no_main]

use const_random::const_random;
use libm::{acosf, cosf, sinf};
use psx::dma;
use psx::dma::{Direction, Step};
use psx::gpu::colors::*;
use psx::gpu::primitives::*;
use psx::gpu::Packet;
use psx::hw::dma::ChannelControl;
use psx::hw::{gpu, irq, Register};
use psx::irq::IRQ;
use psx::sys::rng::Rng;

use framebuffer::{Framebuffer, Result};

fn rotate((px, py): (i16, i16), theta: f32, (cx, cy): (i16, i16)) -> (i16, i16) {
    let (dx, dy) = (px - cx, py - cy);
    let dxf = dx as f32;
    let dyf = dy as f32;
    let newxf = dxf * cosf(theta) - dyf * sinf(theta);
    let newyf = dxf * sinf(theta) + dyf * cosf(theta);
    let newx = newxf as i16;
    let newy = newyf as i16;
    (cx + newx, cy + newy)
}

#[no_mangle]
fn main() -> Result<()> {
    let buf0 = (0, 0);
    let buf1 = (0, 240);
    let res = (320, 240);
    let mut gpu_dma = dma::GPU::new();
    gpu_dma
        .control()
        .set_direction(Direction::FromMemory)
        .set_step(Step::Forward)
        .store();
    let mut fb = Framebuffer::new(buf0, buf1, res)?;
    let mut rng = Rng::new(const_random!(u32));
    let mut quad = Packet::new(PolyG4::new())?;
    let pi = acosf(-1.0);
    let mut theta = 0.0;
    let vertices = [(0, 0), (50, 0), (0, 50), (50, 50)];
    let mut cx = (rng.rand::<u16>() % 320) as i16;
    let mut cy = (rng.rand::<u16>() % 240) as i16;
    let mut min_v = 3;
    let (mut vx, mut vy) = (min_v + rng.rand::<i16>() % 5, min_v + rng.rand::<i16>() % 5);
    quad.payload
        .set_vertices(vertices.map(|(x, y)| (x + cx, y + cy)))
        .set_colors([INDIGO, ORANGE, MINT, YELLOW]);
    irq::Mask::new().enable_irq(IRQ::Vblank).store();
    let mut dir = 1.0;
    loop {
        theta += dir * pi / 100.0;
        if theta >= 2.0 * pi {
            theta -= 2.0 * pi;
        }
        cx += vx;
        cy += vy;
        let new_vertices = vertices
            .map(|v| rotate(v, theta, (25, 25)))
            .map(|(x, y)| (x + cx, y + cy));
        for (x, y) in new_vertices {
            if x >= 320 || x <= 0 {
                vx *= -1;
                dir *= -1.0;
                break;
            }
            if y >= 240 || y <= 0 {
                vy *= -1;
                dir *= -1.0;
                break;
            }
        }
        quad.payload.set_vertices(new_vertices);

        gpu_dma.send_list(&quad)?;
        let mut gpu_stat = gpu::Status::new();
        while !gpu_stat.cmd_ready() || !gpu_stat.dma_ready() {
            gpu_stat.load();
        }
        irq::Status::new()
            .ack(IRQ::Vblank)
            .store()
            .wait(IRQ::Vblank);
        fb.swap(Some(&mut gpu_dma))?;
    }
}
