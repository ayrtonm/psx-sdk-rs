#![no_std]
#![no_main]

use core::mem::size_of;

use psx::bios;
use psx::dma;
use psx::framebuffer::Framebuffer;
use psx::general::{draw_sync, enable_display, reset_graphics, vsync};
use psx::gpu::{Color, Vertex};
use psx::graphics::buffer::DoubleBuffer;
use psx::graphics::ot::DoubleOT;
use psx::graphics::packet::Packet;
use psx::graphics::primitive::PolyG3;
use psx::printer::Printer;
use psx::workarounds::UnwrapUnchecked;

#[no_mangle]
fn main(mut gpu_dma: dma::gpu::CHCR) {
    let gpu_dma = &mut gpu_dma;
    reset_graphics(gpu_dma);
    let mut fb = Framebuffer::new(0, (0, 240), (320, 240), None, gpu_dma);

    let mut p = Printer::new(0, 0, (320, 8), None);
    p.load_font(gpu_dma);

    bios::srand(0xdead_beef);

    const MAX_TRIANGLES: usize = 200;
    const BUF: usize = MAX_TRIANGLES * (size_of::<Packet<PolyG3>>() / 4);
    let buffer = DoubleBuffer::<BUF>::new();
    let mut poly_g3s = unsafe { buffer.poly_g3_array::<MAX_TRIANGLES>().unwrap_unchecked() };
    let mut ot = DoubleOT::default();

    for poly_g3 in &mut poly_g3s {
        ot.insert(poly_g3, 0);
    }
    ot.swap();
    buffer.swap();
    for poly_g3 in &mut poly_g3s {
        ot.insert(poly_g3, 0);
    }

    enable_display();
    const FPS_FACTOR: u32 = 44100 * 768 * 11 / (7 * 3413);
    const FLOAT_FACTOR: u32 = 100;
    let mut fps = 1;
    loop {
        // Split the ordering tables
        let (current_ot, _alt_ot) = ot.split();
        // Send the current OT to the GPU
        let transfer = gpu_dma.send_list(current_ot);
        // Modify polygons in the alt buffer
        buffer.swap();
        for poly_g3 in &mut poly_g3s {
            poly_g3
                .set_vertices(rand_triangle())
                .set_colors(rand_colors());
        }
        // If we wanted to modify the alternate OT, we could do it here using the handle from `split`
        //alt_ot.insert(poly_g3, 0);
        // Swapping ordering tables wouldn't be allowed though
        //ot.swap(); /// ~ERROR
        transfer.wait();
        // After the transfer ends, `&current_ot` is release from the `Transfer` letting us swap the
        // ordering tables.
        ot.swap();
        p.print(
            b"Frame per second: {}",
            [FLOAT_FACTOR * FPS_FACTOR / fps],
            gpu_dma,
        );
        p.reset();
        draw_sync();
        fps = vsync().into();
        fb.swap(gpu_dma);
    }
}

fn rand_vertex() -> Vertex {
    (bios::rand() % 320, bios::rand() % 240).into()
}

fn rand_triangle() -> [Vertex; 3] {
    [rand_vertex(), rand_vertex(), rand_vertex()]
}

fn rand_color() -> Color {
    Color::rgb(bios::rand() as u8, bios::rand() as u8, bios::rand() as u8)
}

fn rand_colors() -> [Color; 3] {
    [rand_color(), rand_color(), rand_color()]
}
