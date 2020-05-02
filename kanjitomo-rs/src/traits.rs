use image::math::Rect;
use crate::area::Point;
use crate::error::KanjitomoError;

pub(crate) trait HasRectangle {
    fn get_rectangle(&self) -> Rect;

    fn get_midpoint(&self) -> Point;
}

pub(crate) trait Task {
    fn run_task(&mut self) -> Result<(), KanjitomoError>;
}

pub(crate) trait Step {
    type Task: Task;

    fn run_step(&mut self) -> Result<(), KanjitomoError>;
}
