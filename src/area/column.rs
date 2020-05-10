use crate::area::Area;
use image::math::Rect;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub(crate) struct Column {
    pub(crate) areas: Vec<Area>,
    rect: Rect,
    vertical: bool,
    furigana: bool,
    furigana_columns: Vec<Column>,
    area_distance: f32,
    score: f32,
    next_column: Option<Rc<RefCell<Column>>>,
    previous_column: Option<Weak<RefCell<Column>>>,
    remove: bool,
}

impl Column {

}

pub(crate) struct ColumnList {
    first: Option<Rc<RefCell<Column>>>,
    last: Option<Rc<RefCell<Column>>>,
}