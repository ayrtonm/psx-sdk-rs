#![no_std]
#![no_main]
#![feature(const_fn_floating_point_arithmetic)]

use core::mem::MaybeUninit;
use psx::sys::gamepad::Gamepad;
use libm::{cosf, sinf};
use psx::constants::*;
use psx::gpu::primitives::PolyFT4;
use psx::gpu::Color;
use psx::gpu::{TexCoord, Vertex};
use psx::include_words;
use psx::{Framebuffer, TIM};

#[no_mangle]
fn main() {
    let mut fb = Framebuffer::default();
    let mut ferris = include_words!("../ferris.tim");
    let ferris = fb.load_tim(TIM::new(&mut ferris).expect("The TIM file is invalid"));

    let mut cube = [PolyFT4::new(); 6];
    for face in &mut cube {
        face.set_tex_page(ferris.tex_page)
            .set_clut(ferris.clut.expect("The TIM file didn't have a CLUT"))
            .set_tex_coords([(0, 0), (0, 85), (128, 0), (128, 85)].map(|(x, y)| TexCoord { x, y }));
    }
    cube[0].set_color(WHITE);
    cube[1].set_color(YELLOW);
    cube[2].set_color(INDIGO);
    cube[3].set_color(ORANGE);
    cube[4].set_color(AQUA);
    cube[5].set_color(LIME);

    let position = [
        XY.sub(Z.div(2.0)),
        XZ.sub(Y.div(2.0)),
        YZ.sub(X.div(2.0)),
        XY.add(Z.div(2.0)),
        XZ.add(Y.div(2.0)),
        YZ.add(X.div(2.0)),
    ];
    let mut theta = 0.0;
    let mut phi = 0.0;
    let mut psi = 0.0;

    let mut buf = MaybeUninit::uninit();
    let mut pad = Gamepad::new(&mut buf);

    loop {
        theta += 0.1;
        phi += 0.1;
        psi += 0.1;
        for b in pad.poll_p1() {
            match b {
                TRIANGLE => theta += 0.5,
                CROSS => theta -= 0.5,
                SQUARE => phi += 0.2,
                CIRCLE => phi -= 0.2,
                _ => (),
            }
        }
        // We want loadable executables to be able to return at some point
        if theta > 10.0 {
            return
        }
        let new_position = position.map(|face| face.Rx(theta).Ry(phi).Rz(psi));
        for (n, face) in cube.iter_mut().enumerate() {
            face.set_vertices(as_vertices(new_position[n]));
            *face.get_color_mut() += INDIGO / 32;
        }
        fb.gp0.send_command(&cube[0]);
        fb.gp0.send_command(&cube[1]);
        fb.draw_sync();
        fb.gp0.send_command(&cube[2]);
        fb.gp0.send_command(&cube[3]);
        fb.draw_sync();
        fb.gp0.send_command(&cube[4]);
        fb.gp0.send_command(&cube[5]);
        fb.draw_sync();
        fb.swap();
        delay(20000);
    }
}

fn as_vertices(pos: Plane) -> [Vertex; 4] {
    pos.points.map(|p3| {
        Vertex::new((
            (p3.x * 64. + p3.z * 32.) as i16,
            (p3.y * 64. + p3.z * 32.) as i16,
        )) + Vertex(128, 128)
    })
}

/// A 2D plane
#[derive(Copy, Clone)]
struct Plane {
    points: [Point; 4],
}

impl Plane {
    fn add(&self, other: Point) -> Plane {
        Plane {
            points: self.points.map(|p| p.add(other)),
        }
    }
    fn sub(&self, other: Point) -> Plane {
        Plane {
            points: self.points.map(|p| p.sub(other)),
        }
    }
    fn Rx(&self, theta: f32) -> Self {
        Self {
            points: self.points.map(|p| Rx(p, theta)),
        }
    }
    fn Ry(&self, theta: f32) -> Self {
        Self {
            points: self.points.map(|p| Ry(p, theta)),
        }
    }
    fn Rz(&self, theta: f32) -> Self {
        Self {
            points: self.points.map(|p| Rz(p, theta)),
        }
    }
}

const XY: Plane = Plane {
    points: [
        X.sub(Y).div(2.0),
        X.add(Y).div(2.0),
        ZERO.sub(Y).sub(X).div(2.0),
        Y.sub(X).div(2.0),
    ],
};
const XZ: Plane = Plane {
    points: [
        X.sub(Z).div(2.0),
        X.add(Z).div(2.0),
        ZERO.sub(Z).sub(X).div(2.0),
        Z.sub(X).div(2.0),
    ],
};
const YZ: Plane = Plane {
    points: [
        Y.sub(Z).div(2.0),
        Y.add(Z).div(2.0),
        ZERO.sub(Z).sub(Y).div(2.0),
        Z.sub(Y).div(2.0),
    ],
};

/// A 3D point
#[derive(Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    const fn add(&self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
    const fn sub(&self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
    const fn div(&self, w: f32) -> Self {
        Point {
            x: self.x / w,
            y: self.y / w,
            z: self.z / w,
        }
    }
}

const ZERO: Point = Point {
    x: 0.,
    y: 0.,
    z: 0.,
};
const X: Point = Point {
    x: 1.,
    y: 0.,
    z: 0.,
};
const Y: Point = Point {
    x: 0.,
    y: 1.,
    z: 0.,
};
const Z: Point = Point {
    x: 0.,
    y: 0.,
    z: 1.,
};

fn Rx(p: Point, theta: f32) -> Point {
    let y = (cosf(theta) * p.y) - (sinf(theta) * p.z);
    let z = (sinf(theta) * p.y) + (cosf(theta) * p.z);
    Point { x: p.x, y, z }
}
fn Ry(p: Point, theta: f32) -> Point {
    let x = (cosf(theta) * p.x) + (sinf(theta) * p.z);
    let z = (-sinf(theta) * p.x) + (cosf(theta) * p.z);
    Point { x, y: p.y, z }
}
fn Rz(p: Point, theta: f32) -> Point {
    let x = (cosf(theta) * p.x) - (sinf(theta) * p.y);
    let y = (sinf(theta) * p.x) + (cosf(theta) * p.y);
    Point { x, y, z: p.z }
}

fn compute_position(pos: Vertex) -> [Vertex; 4] {
    [(-1, -1), (-1, 1), (1, -1), (1, 1)].map(|u| pos + (Vertex::new(u) * 64))
}

fn delay(n: usize) {
    for _ in 0..n {
        unsafe {
            core::ptr::read_volatile(0 as *const u32);
        }
    }
}
