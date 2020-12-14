use core::cmp::{max, min};

use psx::gpu::{Color, Vertex};

#[derive(Clone, Copy)]
pub struct Wall {
    pub start: Vertex,
    pub end: Vertex,
    pub color: Color,
}

impl Wall {
    pub fn new<T, U>(start: T, end: U, color: Color) -> Self
    where
        Vertex: From<T> + From<U>,
    {
        Wall {
            start: Vertex::from(start),
            end: Vertex::from(end),
            color,
        }
    }
    pub fn split(self, intersection: Vertex) -> [Wall; 2] {
        let start = self.start;
        let end = self.end;
        let color = self.color;
        [
            Wall {
                start,
                end: intersection,
                color,
            },
            Wall {
                start: intersection,
                end,
                color,
            },
        ]
    }
    pub fn contains(&self, point: Vertex) -> bool {
        let min_x = min(self.start.x(), self.end.x());
        let max_x = max(self.start.x(), self.end.x());
        let min_y = min(self.start.y(), self.end.y());
        let max_y = max(self.start.y(), self.end.y());
        let (a, b, c) = self.normal_form();
        a * point.x() + b * point.y() + c == 0
            && (min_x..=max_x).contains(&point.x())
            && (min_y..=max_y).contains(&point.y())
            && point != self.start
            && point != self.end
    }
    // Goes from the (x1, y1), (x2, y2) representation to ax + by + c = 0
    // This implicitly extends a line to infinity
    pub fn normal_form(&self) -> (i16, i16, i16) {
        let dx = self.end.x() - self.start.x();
        let dy = self.end.y() - self.start.y();
        let c = (self.end.x() * self.start.y()) - (self.end.y() * self.start.x());
        (dy, -dx, c)
    }
    pub fn slope(&self) -> Vertex {
        let (a, b, _) = self.normal_form();
        (a, b).into()
    }
    pub fn intersect(&self, other: &Wall) -> Option<Vertex> {
        let (a1, b1, c1) = self.normal_form();
        let (a2, b2, c2) = other.normal_form();
        if a1 * b2 != a2 * b1 {
            let x = (b1 * c2 - b2 * c1) / (a1 * b2 - a2 * b1);
            let y = (a2 * c1 - a1 * c2) / (a1 * b2 - a2 * b1);
            let intersection = (x, y).into();
            other.contains(intersection).then_some(intersection)
        } else {
            None
        }
    }
    pub fn behind(&self, other: &Vertex) -> bool {
        (other.x() - self.start.x()) * (self.end.y() - self.start.y())
            - (other.y() - self.start.y()) * (self.end.x() - self.start.x())
            < 0
    }
    pub fn y(&self, x: i16) -> Option<i16> {
        let (a, b, c) = self.normal_form();
        (b != 0).then_some(-(a * x + c) / b)
    }
    pub fn x(&self, y: i16) -> Option<i16> {
        let (a, b, c) = self.normal_form();
        (a != 0).then_some(-(b * y + c) / a)
    }
}
