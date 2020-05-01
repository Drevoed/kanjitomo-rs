use super::Point;
use image::math::Rect;

pub(crate) trait HasRectangle {
    fn get_rectangle(&self) -> Rect;

    fn get_midpoint(&self) -> Point;
}
