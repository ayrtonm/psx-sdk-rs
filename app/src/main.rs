#![feature(core_intrinsics)]
#![feature(array_map)]
#![no_std]
#![no_main]

#[allow(unused_imports)]
#[macro_use]
extern crate core;

use libpsx::gpu::{Palette, Position, Color, Opacity, draw_polygon, draw_rect};
use libpsx::allocator::BiosAllocator;
use libpsx::util::delay;

#[no_mangle]
pub fn main() {
    BiosAllocator::init();
    let mut theta = 0.0;
    let delta = 1.0;
    let size = 256;
    // Clear command FIFO
    libpsx::bios::gpu_gp1_command_word(0x01000000);
    // Top left at 0,0
    libpsx::bios::gpu_command_word(0xe3000000);
    // Bottom right: 256x256
    libpsx::bios::gpu_command_word(0xe4080100);
    // Offset at 0,0
    libpsx::bios::gpu_command_word(0xe5000000);
    loop {
        theta += delta;
        if theta > 360.0 {
            theta = 0.0;
        };
        draw_rect(&Position::zero(), size, size, &Color::black(), &Opacity::Opaque);
        draw(theta);
        blink();
    }
}

fn draw(theta: f32) {
    // Shaded quad
    let offset = 64;
    let size = 128;
    let center = Position::new(128, 128);
    let offset = Position::new(offset, offset);
    let pos = Position::rectangle(offset, size, size)
                        .map(|p| rotate_point(p, theta, center));
    let pal = Palette::Shaded([Color::aqua(),
                               Color::mint(),
                               Color::indigo(),
                               Color::orange()]);
    draw_polygon(&pos, &pal, &Opacity::Opaque);
}

fn blink() {
    delay(50000);
}

// Does the GTE expose trig functions directly?
fn sin(mut x: f32) -> f32 {
    fn approx_sin(z: f32) -> f32 {
        4.0 * z * (180.0 - z) / (40500.0 - (z * (180.0 - z)))
    }
    while x < 0.0 {
        x += 360.0;
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
fn rotate_point(p: Position, theta: f32, c: Position) -> Position {
    let dx = p.x() as f32 - c.x() as f32;
    let dy = p.y() as f32 - c.y() as f32;
    let xp = dx * cos(theta) - dy * sin(theta);
    let yp = dy * cos(theta) + dx * sin(theta);
    let xf = xp + c.x() as f32;
    let yf = yp + c.y() as f32;
    Position::new(xf as u16, yf as u16)
}
