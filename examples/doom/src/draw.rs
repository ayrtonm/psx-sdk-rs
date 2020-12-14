use super::{Wall, IO};

use alloc::vec::Vec;
use core::cmp::{max, min};
use core::ops::RangeInclusive;

use psx::gpu::graphics::{SingleBuffer, SingleOT};
use psx::gpu::{Color, Vertex};

fn resize_aerial(v: Vertex) -> Vertex {
    v.scale(3).shift((20, 15))
}

fn resize_first_person(v: Vertex) -> Vertex {
    v.scale(20).shift((160, 120))
}

fn transform_first_person(start: Vertex, end: Vertex) -> [Vertex; 4] {
    let infinity = -4;
    let ceiling = |y| (y - infinity);
    let floor = |y| -1 * (y - infinity);
    [
        (start.x(), ceiling(start.y())),
        (start.x(), floor(start.y())),
        (end.x(), ceiling(end.y())),
        (end.x(), floor(end.y())),
    ]
    .map(|p| resize_first_person(p.into()))
}

pub fn draw(walls: &Vec<&Wall>, io: &mut IO) {
    let otc_dma = &mut io.otc_dma;
    let gpu_dma = &mut io.gpu_dma;
    let gp1 = &mut io.gp1;

    let buffer = SingleBuffer::<1024>::new();
    let mut ot = SingleOT::<3>::new();
    otc_dma.clear(&ot).wait();

    let mut ranges_drawn = Vec::<RangeInclusive<i16>>::new();
    let mut draw_wall = |wall: &Wall| {
        let prim = buffer.LineF2().unwrap();
        prim.color(wall.color)
            .vertices([wall.start, wall.end].map(|v| resize_aerial(v)));
        ot.add_prim(prim, 0);
        let hud = buffer.PolyF4().unwrap();
        let hud_x = 50;
        let hud_y = 50;
        hud.color(Color::BLACK)
            .vertices([(0, 0), (hud_x, 0), (0, hud_y), (hud_x, hud_y)]);
        ot.add_prim(hud, 1);

        let mut start_x = min(wall.start.x(), wall.end.x());
        let mut end_x = max(wall.end.x(), wall.start.x());
        for rng in &ranges_drawn {
            if rng.contains(&start_x) {
                start_x = *rng.end();
            }
            if rng.contains(&end_x) {
                end_x = *rng.start();
            }
        }
        ranges_drawn.push(start_x..=end_x);
        let start = (start_x, wall.y(start_x).unwrap_or(wall.start.y())).into();
        let end = (end_x, wall.y(end_x).unwrap_or(wall.end.y())).into();
        let prim = buffer.PolyF4().unwrap();
        prim.color(wall.color)
            .vertices(transform_first_person(start, end));
        ot.add_prim(prim, 2);
    };
    for wall in walls {
        draw_wall(wall);
    }
    gpu_dma.prepare_ot(gp1).send(&ot).wait();
}
