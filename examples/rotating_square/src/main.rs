#![no_std]
#![no_main]
#![feature(array_map)]

use core::cell::RefCell;
use libpsx::gpu::vertex::Vertex;
use libpsx::gpu::color::Color;
use libpsx::gpu::framebuffer::Framebuffer;
use libpsx::gpu::res::{Hres, Vres};

libpsx::exe!();

fn main(mut ctxt: Ctxt) {
    // This will give an error since there should only be one instance of Ctxt
    //let fake_ctxt = crate::executable::ctxt;
    let mut theta = 0.0;
    let delta = 0.0625;
    let draw_env = RefCell::new(ctxt.take_draw_env().expect("DrawEnv has been taken"));
    let display_env = RefCell::new(ctxt.take_display_env().expect("DisplayEnv has been taken"));
    let buf0 = (0, 0);
    let buf1 = (0, 240);
    let res = (Hres::H320, Vres::V240);
    display_env.borrow_mut().reset_gpu();
    let mut fb = Framebuffer::new(&draw_env, &display_env, buf0, buf1, res);
    loop {
        theta += delta;
        while theta > 360.0 {
            theta -= 360.0;
        }
        let (quad, pal) =  draw(theta);
        draw_env.borrow_mut().draw_shaded_quad(&quad, &pal);
        fb.swap();
    }
}

fn draw(theta: f32) -> ([Vertex; 4], [Color; 4]) {
    let center = &Vertex::new(160, 120);
    let size = 128;
    let square = Vertex::square(center, size).map(|p| rotate_point(p, theta, center));
    let palette = [Color::aqua(), Color::mint(), Color::indigo(), Color::orange()];
    (square, palette)
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
fn rotate_point(p: Vertex, theta: f32, c: &Vertex) -> Vertex {
    let dx = p.x() as f32 - c.x() as f32;
    let dy = p.y() as f32 - c.y() as f32;
    let xp = dx * cos(theta) - dy * sin(theta);
    let yp = dy * cos(theta) + dx * sin(theta);
    let xf = xp + c.x() as f32;
    let yf = yp + c.y() as f32;
    Vertex::new(xf as u16, yf as u16)
}
