#![no_std]
#![no_main]
#![feature(array_map)]

use psx::framebuffer::UnsafeFramebuffer;
use psx::gpu::graphics::primitive::PolyG4;
use psx::gpu::graphics::{packet_size, DoubleBuffer, DoubleOT};
use psx::gpu::{Color, Pixel, Vertex};
use psx::interrupt::IRQ;
use psx::mmio::MMIO;

#[no_mangle]
fn main(mut mmio: MMIO) {
    // Borrow all the IO ports we'll need
    let dma_control = &mut mmio.dma_control;
    let otc_dma = &mut mmio.otc_dma.enable(dma_control);
    let gpu_dma = &mut mmio.gpu_dma.enable(dma_control);
    let gp1 = &mut mmio.gp1;
    let gpu_stat = &mut mmio.gpu_stat;
    let int_mask = &mut mmio.int_mask;
    let int_stat = &mut mmio.int_stat;

    // Construct some higher-level utilities
    let mut fb = UnsafeFramebuffer::default();
    const N: usize = packet_size::<PolyG4>();
    let buffer = DoubleBuffer::<N>::new();
    let mut ot = DoubleOT::<1>::new();

    // Initialize *both* ordering tables
    otc_dma.clear(&ot).wait();
    otc_dma.clear(&ot.swap()).wait();

    let x = 64;
    let y = 128;
    let midpoint = (x + y) / 2;
    let init = [(x, x), (y, x), (x, y), (y, y)];
    // Make a double-buffered packet
    let mut poly = buffer.polyg4().unwrap();

    // Initialize one copy of the packet as a blue rectangle
    let pal = [Color::AQUA, Color::MINT, Color::INDIGO, Color::ORANGE];
    poly.vertices(init).color(pal);
    // Insert that packet into an ordering table
    ot.insert(&mut poly, 0);
    // Switch over to the other prim buffer
    buffer.swap();
    // Initialize the other copy of the packet as an orange rectangle
    poly.vertices(init).color(pal);
    // Insert that packet into the other ordering table
    ot.swap();
    ot.insert(&mut poly, 0);

    //Let's start by sending buffer 1
    buffer.swap();
    ot.swap();
    let mut theta = 0.0;
    gpu_dma.prepare_ot(gp1);
    int_mask.enable(IRQ::Vblank);
    loop {
        // Send an ordering table
        // While the ordering table is being sent to the GPU, we can keep working if
        // chopping is on We can keep working while ot[i] is being sent if
        let send_ot = gpu_dma.send(&ot);
        buffer.swap();
        theta += 1.0;
        if theta == 360.0 {
            theta = 0.0;
        };
        poly.vertices(init.map(|v| rotate_point(v, theta, midpoint)));
        ot.swap();
        send_ot.wait();
        gpu_stat.sync();
        int_stat.ack(IRQ::Vblank);
        int_stat.wait(IRQ::Vblank);
        fb.swap();
    }
}

fn sin(mut x: f32) -> f32 {
    fn approx_sin(z: f32) -> f32 {
        4.0 * z * (180.0 - z) / (40500.0 - (z * (180.0 - z)))
    }
    while x < 0.0 {
        x += 360.0;
    }
    while x > 360.0 {
        x -= 360.0;
    }
    if x <= 180.0 {
        approx_sin(x)
    } else {
        -approx_sin(x - 180.0)
    }
}

fn cos(x: f32) -> f32 {
    let y = 90.0 - x;
    sin(y)
}

// Rotation is better handled by the GTE but this'll do for a demo
fn rotate_point<T, U>(p: T, theta: f32, c: U) -> (Pixel, Pixel)
where Vertex: From<T> + From<U> {
    let p = Vertex::from(p);
    let c = Vertex::from(c);
    let dx = p.x() as f32 - c.x() as f32;
    let dy = p.y() as f32 - c.y() as f32;
    let xp = dx * cos(theta) - dy * sin(theta);
    let yp = dy * cos(theta) + dx * sin(theta);
    let xf = xp + c.x() as f32;
    let yf = yp + c.y() as f32;
    (xf as Pixel, yf as Pixel)
}
