mod area;
mod area_task;
mod column;

pub(crate) use area::Area;
pub(crate) use area_task::AreaTask;
pub(crate) use column::Column;
use sealed::*;

pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    #[inline(always)]
    pub fn distance(&self, point: &Point) -> f32 {
        distance(self.x as f32, self.y as f32, point.x as f32, point.y as f32)
    }
}

mod sealed {
    use std::f32;

    #[inline(always)]
    pub(super) fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        distance_sq(x1, y1, x2, y2).sqrt()
    }

    #[inline(always)]
    pub(super) fn distance_sq(x1: f32, y1: f32, mut x2: f32, mut y2: f32) -> f32 {
        x2 -= x1;
        y2 -= y1;
        x2 * x2 + y2 * y2
    }
}
