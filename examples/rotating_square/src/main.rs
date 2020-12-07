#![no_std]
#![no_main]
#![feature(array_map)]

use psx::framebuffer::UnsafeFramebuffer;
use psx::gpu::prim::{Buffer, DoubleBuffer, DoubleOT, OT};
use psx::gpu::{Color, Pixel, Vertex};
use psx::interrupt::IRQ;

psx::exe!();

fn main(mut mmio: MMIO) {
    // Borrow all the IO ports we'll need
    let dma_control = &mut mmio.dma_control;
    let otc_dma = &mut mmio.otc_dma;
    let gpu_dma = &mut mmio.gpu_dma;
    let gp1 = &mut mmio.gp1;
    let gpu_stat = &mut mmio.gpu_stat;
    let int_mask = &mut mmio.int_mask;
    let int_stat = &mut mmio.int_stat;

    // Construct some higher-level utilities
    let mut fb = UnsafeFramebuffer::default();
    let buffer = DoubleBuffer::<100>::new();
    let mut ot = DoubleOT::<1>::new();

    // Enable the DMA channels
    dma_control.otc(true).gpu(true);
    // Initialize *both* ordering tables
    otc_dma.clear(&*ot).wait();
    otc_dma.clear(&*ot.swap()).wait();

    let x = 25;
    let y = 50;
    let init = [(x, x), (y + 20, x), (x, y), (y + 20, y)];
    // Make a double-buffered packet
    let mut poly = buffer.PolyF4().unwrap();

    // Initialize one copy of the packet as a blue rectangle
    poly.vertices(init).color(Color::AQUA);
    // Insert that packet into an ordering table
    ot.add_prim(&mut *poly, 0);
    // Switch over to the over prim buffer
    buffer.swap();
    // Initialize the other copy of the packet as an orange rectangle
    poly.vertices(init).color(Color::ORANGE);
    // Insert that packet into the other ordering table
    ot.swap();
    ot.add_prim(&mut *poly, 0);

    // Let's start by sending buffer 1
    buffer.swap();
    ot.swap();
    gpu_dma.prepare_ot(gp1);
    let mut theta = 0.0;
    //int_mask.enable(IRQ::Vblank);
    loop {
        // Send an ordering table
        // While the ordering table is being sent to the GPU, we can keep working if
        // chopping is on We can keep working while ot[i] is being sent if
        let send_ot = gpu_dma.send(&*ot);
        buffer.swap();
        theta += 15.0;
        poly.vertices(init.map(|v| rotate_point(v, theta, ((x + y) / 2, (x + y) / 2))));
        ot.swap();
        send_ot.wait();
        gpu_stat.sync();
        psx::delay(1000000);
        fb.swap();
        //int_stat.wait(IRQ::Vblank);
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
fn rotate_point<T, U>(p: T, theta: f32, c: U) -> Vertex
where Vertex: From<T> + From<U> {
    let p = Vertex::from(p);
    let c = Vertex::from(c);
    let dx = p.x() as f32 - c.x() as f32;
    let dy = p.y() as f32 - c.y() as f32;
    let xp = dx * cos(theta) - dy * sin(theta);
    let yp = dy * cos(theta) + dx * sin(theta);
    let xf = xp + c.x() as f32;
    let yf = yp + c.y() as f32;
    (xf as Pixel, yf as Pixel).into()
}
