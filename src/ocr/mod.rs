use image::math::Rect;

mod ocr_result;
mod ocr_task;
mod ocr_manager;
mod transform;

pub(crate) use ocr_result::OCRResult;
use std::collections::HashMap;

pub struct OCR {

}

#[derive(Default, Clone, Debug)]
pub(crate) struct TargetMatrix {
    matrix: Vec<u8>,
    pixels: u32,
    halo: Vec<Vec<u8>>,
    char_index: u32,
    transform: Transformation,
}

impl TargetMatrix {
    pub(crate) fn new(
        matrix: Vec<u8>,
        pixels: u32,
        halo: Vec<Vec<u8>>,
        char_index: u32,
        transform: Transformation
    ) -> Self {
        Self {
            matrix,
            pixels,
            halo,
            char_index,
            transform
        }
    }
}

#[derive(Default, Clone, Debug)]
pub(crate) struct ReferenceMatrix {
    pub(crate) character: char,
    matrix: Vec<u8>,
    pixels: u32,
    halo: Vec<Vec<u8>>,
    score_modifier: f32,
    font_name: String,
    components: Vec<Component>,
    transformations: Vec<Transformation>,
}

pub(crate) struct ReferenceMatrixCacheLoader {

}

pub(crate) struct ReferenceMatrixCache {
    cache: HashMap<String, Vec<ReferenceMatrix>>
}

impl ReferenceMatrixCache {

}

#[derive(Default, Debug, Clone)]
pub(crate) struct Component {
    bounds: Option<Rect>,
    matrix: Vec<u8>,
    pixels: u32,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Transformation {
    horizontal_translate: i32,
    vertical_translate: i32,
    horizontal_stretch: i32,
    vertical_stretch: i32,
}

impl Transformation {
    pub(crate) fn new(h_t: i32, v_t: i32, h_s: i32, v_s: i32) -> Self {
        Self {
            horizontal_translate: h_t,
            horizontal_stretch: h_s,
            vertical_translate: v_t,
            vertical_stretch: v_s,
        }
    }

    pub(crate) fn contains(&self, h_t: i32, v_t: i32, h_s: i32, v_s: i32) -> bool {
        (self.horizontal_translate == h_t
            && self.vertical_translate == v_t
            && self.horizontal_stretch == h_s
            && self.vertical_stretch == v_s)
    }
}
