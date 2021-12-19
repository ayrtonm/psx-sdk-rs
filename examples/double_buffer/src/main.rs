#![no_std]
#![no_main]

use libm::{acosf, cosf, sinf};
use psx::gpu::Packet;
use psx::dma;
use psx::dma::{Direction, Step};
use psx::gpu::colors::*;
use psx::gpu::primitives::*;
use psx::hw::dma::ChannelControl;
use psx::hw::{gpu, irq, Register};
use psx::irq::IRQ;
use psx::{println, Result};

mod framebuffer;
use framebuffer::Framebuffer;

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
    let mut quad = Packet::new(PolyG4::new())?;
    let pi = acosf(-1.0);
    let mut theta = 0.0;
    let vertices = [(150, 150), (200, 150), (150, 200), (200, 200)];
    let (mut cx, mut cy) = (0, 0);
    for (x, y) in vertices {
        cx += x / 4;
        cy += y / 4;
    }
    let center = (cx, cy);
    quad.payload.set_vertices(vertices)
        .set_colors([INDIGO, ORANGE, MINT, YELLOW]);
    irq::Mask::new().enable_irq(IRQ::Vblank).store();
    loop {
        theta += pi / 100.0;
        if theta >= 2.0 * pi {
            theta -= 2.0 * pi;
        }
        quad.payload.set_vertices(vertices.map(|v| rotate(v, theta, center)));

        fb.gp0.send_command(&quad.payload);
        //gpu_dma.send_list_and(&quad, || ())?;
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
