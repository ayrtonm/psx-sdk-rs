use core::ops::{Add, Div, Sub};
use libm::{cosf, sinf};

type Unit = f32;
const ZERO: Unit = 0.0;
const ONE: Unit = 1.0;
const TWO: Unit = 2.0;

#[derive(Copy, Clone)]
pub struct Cube {
    pub faces: [Plane; 6],
}

#[derive(Copy, Clone)]
pub struct Plane {
    pub points: [Point; 4],
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn zero() -> Self {
        Point {
            x: ZERO,
            y: ZERO,
            z: ZERO,
        }
    }
    pub fn x() -> Self {
        Point {
            x: ONE,
            y: ZERO,
            z: ZERO,
        }
    }
    pub fn y() -> Self {
        Point {
            x: ZERO,
            y: ONE,
            z: ZERO,
        }
    }
    pub fn z() -> Self {
        Point {
            x: ZERO,
            y: ZERO,
            z: ONE,
        }
    }
}

impl Plane {
    pub fn xy() -> Self {
        Plane {
            points: [
                Point::zero(),
                Point::x(),
                Point::y(),
                Point::x() + Point::y(),
            ],
        }
    }
    pub fn yz() -> Self {
        Plane {
            points: [
                Point::zero(),
                Point::z(),
                Point::y(),
                Point::y() + Point::z(),
            ],
        }
    }
    pub fn zx() -> Self {
        Plane {
            points: [
                Point::zero(),
                Point::z(),
                Point::x(),
                Point::z() + Point::x(),
            ],
        }
    }
    pub fn rx(&self, theta: f32) -> Self {
        Plane {
            points: self.points.map(|point| rx(point, theta)),
        }
    }
    pub fn ry(&self, theta: f32) -> Self {
        Plane {
            points: self.points.map(|point| ry(point, theta)),
        }
    }
    pub fn rz(&self, theta: f32) -> Self {
        Plane {
            points: self.points.map(|point| rz(point, theta)),
        }
    }
}

impl Add<Point> for Plane {
    type Output = Plane;
    fn add(self, other: Point) -> Plane {
        Plane {
            points: self.points.map(|point| point + other),
        }
    }
}

impl Sub<Point> for Plane {
    type Output = Plane;
    fn sub(self, other: Point) -> Plane {
        Plane {
            points: self.points.map(|point| point - other),
        }
    }
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Div<f32> for Point {
    type Output = Point;
    fn div(self, other: f32) -> Point {
        Point {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Cube {
    pub fn new() -> Self {
        let offset = (Point::x() + Point::y() + Point::z()) / TWO;
        let unit_cube = [
            Plane::xy(),
            Plane::yz(),
            Plane::zx(),
            Plane::xy() + Point::z(),
            Plane::yz() + Point::x(),
            Plane::zx() + Point::y(),
        ];
        let faces = unit_cube.map(|plane| plane - offset);
        Cube { faces }
    }
}

fn rx(p: Point, theta: f32) -> Point {
    let y = (cosf(theta) * p.y) - (sinf(theta) * p.z);
    let z = (sinf(theta) * p.y) + (cosf(theta) * p.z);
    Point { x: p.x, y, z }
}
fn ry(p: Point, theta: f32) -> Point {
    let x = (cosf(theta) * p.x) + (sinf(theta) * p.z);
    let z = (-sinf(theta) * p.x) + (cosf(theta) * p.z);
    Point { x, y: p.y, z }
}
fn rz(p: Point, theta: f32) -> Point {
    let x = (cosf(theta) * p.x) - (sinf(theta) * p.y);
    let y = (sinf(theta) * p.x) + (cosf(theta) * p.y);
    Point { x, y, z: p.z }
}
