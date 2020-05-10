use crate::Rect;

mod ocr_result;
mod ocr_task;
mod ocr_manager;
mod transform;

pub(crate) use ocr_result::OCRResult;
pub(crate) use ocr_manager::OCRManager;
use std::collections::{HashMap, HashSet};
use std::hash::{Hasher, BuildHasherDefault, Hash};
use std::fmt::Formatter;
use serde::Serialize;
use nalgebra::DMatrix;
use crate::util::matrix_util::is_bit_set;
use bit::BitIndex;

pub struct OCR {

}

#[derive(Default, Clone, Debug)]
pub(crate) struct TargetMatrix {
    matrix: [u32; 32],
    pixels: u32,
    halo: Vec<[u32; 32]>,
    char_index: u32,
    transform: Transformation,
}

impl TargetMatrix {
    pub(crate) fn new(
        matrix: [u32; 32],
        pixels: u32,
        halo: Vec<[u32; 32]>,
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
    cache: Option<ReferenceMatrixCache>,
}

impl ReferenceMatrixCacheLoader {
    pub(crate) fn new() -> Self {
        Self {
            cache: None,
        }
    }

    pub(crate) fn load(&mut self) {
        match self.cache {
            None => (),
            Some(ref mut cache) => {

            }
        }
    }

    pub(crate) fn deserialize(&mut self) {

    }
}

pub(crate) struct ReferenceMatrixCache {
    cache: HashMap<String, Vec<ReferenceMatrix>>
}

impl ReferenceMatrixCache {

}

pub struct ReferenceMatrixCacheBuilder {
    chars: HashSet<char>
}

pub(crate) struct ComponentBuilder {

}

pub(crate) struct ComponentFindUnconnected {
    pixels: Vec<Pixel>,
    todo: Vec<Pixel>,
    visited: DMatrix<bool>,
    bounds: Rect,
    matrix: [u32; 32]
}

impl ComponentFindUnconnected {
    pub(crate) fn new(component: Component) -> Self {
        Self {
            matrix: component.matrix,
            bounds: component.bounds,
            visited: DMatrix::from_element(32, 32, false),
            todo: vec![],
            pixels: vec![]
        }
    }

    pub(crate) fn run(&mut self) -> Vec<Component> {
        let mut components: Vec<Component> = vec![];

        for x in self.bounds.x..self.bounds.x + self.bounds.width {
            for y in self.bounds.y..self.bounds.y + self.bounds.height {
                self.todo.push(Pixel::new(x, y));

                while let Some(px) = self.todo.pop() {
                    self.check_pixel(px)
                }

                if self.pixels.len() == 0 { continue; }
                components.push(self.build_new_component());
                self.pixels.clear();
            }
        }

        components
    }

    fn check_pixel(&mut self, px: Pixel) {
        if self.visited[(px.y as usize, px.x as usize)] {
            return;
        }

        if is_bit_set(px.x, px.y, &self.matrix) {
            self.pixels.push(px);
            self.todo.push(Pixel::new(px.x - 1, px.y));
            self.todo.push(Pixel::new(px.x + 1, px.y));
            self.todo.push(Pixel::new(px.x, px.y - 1));
            self.todo.push(Pixel::new(px.x, px.y + 1));
        }

        self.visited[(px.y as usize, px.x as usize)] = true;
    }

    fn build_new_component(&mut self) -> Component {
        let mut component = Component::default();

        let mut min_x = 31;
        let mut min_y = 31;
        let mut max_x = 0;
        let mut max_y = 0;

        for px in &self.pixels {
            component.matrix[px.y as usize].set_bit(px.x as usize, true);
            if px.x < min_x { min_x = px.x };
            if px.y < min_y { min_y = px.y };
            if px.x > max_x { max_x = px.x };
            if px.y > max_y { max_y = px.y };
        }
        component.pixels = self.pixels.len() as u32;
        component.bounds = Rect {
            x: min_x,
            y: min_y,
            width: max_x - min_x + 1,
            height: max_y - min_y + 1
        };

        component
    }
}


#[derive(Copy, Clone, Eq, PartialEq)]
struct Pixel {
    x: u32,
    y: u32,
}

impl Pixel {
    fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y
        }
    }
}

impl Hash for Pixel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.x + (100000 * self.y))
    }
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Default, Debug, Clone, Serialize)]
pub(crate) struct Component {
    bounds: Rect,
    matrix: [u32; 32],
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
