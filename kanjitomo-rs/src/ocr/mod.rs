use image::math::Rect;

mod ocr_result;
mod ocr_task;

const R_M_SERIAL_VERSION_UID: u64 = 3;
const C_SERIAL_VERSION_UID: u64 = 2;
const T_SERIAL_VERSION_UID: u64 = 1;

#[derive(Default, Clone, Debug)]
pub(crate) struct TargetMatrix {
    matrix: Vec<u8>,
    pixels: u32,
    halo: Vec<Vec<u8>>,
    char_index: u32,
    transform: Transformation
}

#[derive(Default, Clone, Debug)]
pub(crate) struct ReferenceMatrix {
    character: char,
    matrix: Vec<u8>,
    pixels: u32,
    halo: Vec<Vec<u8>>,
    score_modifier: f32,
    font_name: String,
    components: Vec<Component>,
    transformations: Vec<Transformation>,
}

#[derive(Default, Debug, Clone)]
pub(crate) struct Component {
    bounds: Option<Rect>,
    matrix: Vec<u8>,
    pixels: u32,
}

#[derive(Default, Debug, Clone)]
pub(crate) struct Transformation {
    horizontal_translate: u32,
    vertical_translate: u32,
    horizontal_stretch: u32,
    vertical_stretch: u32,
}

impl Transformation {
    pub(crate) fn new(h_t: u32, v_t: u32, h_s: u32, v_s: u32) -> Self {
        Self {
            horizontal_translate: h_t,
            horizontal_stretch: h_s,
            vertical_translate: v_t,
            vertical_stretch: v_s,
        }
    }

    pub(crate) fn contains(&self, h_t: u32, v_t: u32, h_s: u32, v_s: u32) -> bool {
        (self.horizontal_translate == h_t
            && self.vertical_translate == v_t
            && self.horizontal_stretch == h_s
            && self.vertical_stretch == v_s)
    }
}