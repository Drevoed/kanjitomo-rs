use crate::area::Area;
use image::math::Rect;

#[derive(Debug, Clone)]
pub struct Column {
    areas: Vec<Area>,
    rect: Rect,
    vertical: bool,
    furigana: bool,
    furigana_columns: Vec<Column>,
}
