#![no_std]
#![no_main]
#![feature(array_map)]

use psx::framebuffer::UnsafeFramebuffer;
use psx::gpu::prim::{size_of, DoubleBuffer, DoubleOT, PolyG3};
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
    // Size the buffer so it fits exactly 50 (double-buffered) PolyG3s
    const T_NUM: usize = 50;
    const BUFFER_SIZE: usize = size_of::<PolyG3>(T_NUM);
    let buffer = DoubleBuffer::<BUFFER_SIZE>::new();
    let mut ot = DoubleOT::<1>::new();

    // Enable the DMA channels
    dma_control.otc(true).gpu(true);
    // Initialize *both* ordering tables
    otc_dma.clear(&ot).wait();
    otc_dma.clear(&ot.swap()).wait();

    let position = |i, theta, a| {
        [(0.0, 0.0), (a / 2.0, sin(60.0) * a), (a, 0.0)]
            .map(|(x, y)| ((x - a / 2.0) as Pixel, (y - sin(60.0) * a / 2.0) as Pixel))
            .map(|v| rotate_point(v, theta, 0).shift(i as Pixel + 10))
    };
    // Allocate 50 double-buffered PolyG3s. Note the array `triangles` below only holds handles to
    // the allocated PolyG3s. The PolyG3s themselves are in the buffer's backing arrays.
    let mut triangles = buffer.array::<PolyG3, T_NUM>().unwrap();

    // Colors will be constant within the loop, so let's initialize them now. Since the PolyG3s are
    // double-buffered, the colors must be initialized for both copies. Let's use a closure to
    // simplify this.
    let mut init_packet = |ot: &mut DoubleOT<1>| {
        for i in 0..T_NUM {
            triangles[i].color([Color::RED, Color::GREEN, Color::BLUE]);
            // Don't forget to insert the triangles into onne of ther ordering tables
            ot.add_prim(&mut triangles[i], 0);
        }
    };
    init_packet(&mut ot);
    buffer.swap();
    ot.swap();
    init_packet(&mut ot);

    let mut theta = 0.0;
    gpu_dma.prepare_ot(gp1);
    int_mask.enable(IRQ::Vblank);
    loop {
        // Send an ordering table
        let send_ot = gpu_dma.send(&ot);
        theta += 5.0;
        if theta == 360.0 {
            theta = 0.0;
        };
        // Rotate the triangles in the other buffer
        buffer.swap();
        for i in 0..T_NUM {
            triangles[i].vertices(position(i, theta, 10.0));
        }
        // Swap the ordering tables for the next frame
        ot.swap();
        // If we needed to rearrange anything within the OT, we'd do it here

        // Wait until the DMA transfer is done
        send_ot.wait();
        // Wait until the GPU is done
        gpu_stat.sync();
        // Wait until the next vblank
        int_stat.ack(IRQ::Vblank);
        int_stat.wait(IRQ::Vblank);
        // Show the ordering table we sent at the beginning of the loop
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
fn rotate_point<T, U>(p: T, theta: f32, c: U) -> Vertex
where
    Vertex: From<T> + From<U>,
{
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
