#![no_std]
#![no_main]
#![feature(const_fn_floating_point_arithmetic)]

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
    cube[0].set_color(WHITE.into());
    cube[1].set_color(YELLOW.into());
    cube[2].set_color(INDIGO.into());
    cube[3].set_color(ORANGE.into());
    cube[4].set_color(AQUA.into());
    cube[5].set_color(LIME.into());

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

    loop {
        theta += 0.1;
        phi += 0.1;
        psi += 0.1;
        // We want loadable executables to be able to return at some point
        if theta > 10.0 {
            return
        }
        let new_position = position.map(|face| face.Rx(theta).Ry(phi).Rz(psi));
        for (n, face) in cube.iter_mut().enumerate() {
            face.set_vertices(as_vertices(new_position[n]));
            // The API for colors could use some work...
            let dim_indigo = INDIGO.halve().halve().halve().halve().halve();
            face.set_color(Color::from(face.get_color()).sum(dim_indigo).into());
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
        delay(200000);
    }
}

fn as_vertices(pos: F2) -> [Vertex; 4] {
    pos.points.map(|p3| {
        Vertex::new((
            (p3.x * 64. + p3.z * 32.) as i16,
            (p3.y * 64. + p3.z * 32.) as i16,
        )) + Vertex(128, 128)
    })
}

/// A 2D plane
#[derive(Copy, Clone)]
struct F2 {
    points: [P3; 4],
}

impl F2 {
    fn add(&self, other: P3) -> F2 {
        F2 {
            points: self.points.map(|p| p.add(other)),
        }
    }
    fn sub(&self, other: P3) -> F2 {
        F2 {
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

const XY: F2 = F2 {
    points: [
        X.sub(Y).div(2.0),
        X.add(Y).div(2.0),
        ZERO.sub(Y).sub(X).div(2.0),
        Y.sub(X).div(2.0),
    ],
};
const XZ: F2 = F2 {
    points: [
        X.sub(Z).div(2.0),
        X.add(Z).div(2.0),
        ZERO.sub(Z).sub(X).div(2.0),
        Z.sub(X).div(2.0),
    ],
};
const YZ: F2 = F2 {
    points: [
        Y.sub(Z).div(2.0),
        Y.add(Z).div(2.0),
        ZERO.sub(Z).sub(Y).div(2.0),
        Z.sub(Y).div(2.0),
    ],
};

/// A 3D point
#[derive(Copy, Clone)]
struct P3 {
    x: f32,
    y: f32,
    z: f32,
}

impl P3 {
    const fn add(&self, other: P3) -> P3 {
        P3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
    const fn sub(&self, other: P3) -> P3 {
        P3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
    const fn div(&self, w: f32) -> Self {
        P3 {
            x: self.x / w,
            y: self.y / w,
            z: self.z / w,
        }
    }
}

const ZERO: P3 = P3 {
    x: 0.,
    y: 0.,
    z: 0.,
};
const X: P3 = P3 {
    x: 1.,
    y: 0.,
    z: 0.,
};
const Y: P3 = P3 {
    x: 0.,
    y: 1.,
    z: 0.,
};
const Z: P3 = P3 {
    x: 0.,
    y: 0.,
    z: 1.,
};

fn Rx(p: P3, theta: f32) -> P3 {
    let y = (cosf(theta) * p.y) - (sinf(theta) * p.z);
    let z = (sinf(theta) * p.y) + (cosf(theta) * p.z);
    P3 { x: p.x, y, z }
}
fn Ry(p: P3, theta: f32) -> P3 {
    let x = (cosf(theta) * p.x) + (sinf(theta) * p.z);
    let z = (-sinf(theta) * p.x) + (cosf(theta) * p.z);
    P3 { x, y: p.y, z }
}
fn Rz(p: P3, theta: f32) -> P3 {
    let x = (cosf(theta) * p.x) - (sinf(theta) * p.y);
    let y = (sinf(theta) * p.x) + (cosf(theta) * p.y);
    P3 { x, y, z: p.z }
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
