use crate::ocr::ocr_task::OCRTask;
use std::collections::HashMap;
use crate::ocr::Transformation;
use crate::util::{stretch_check_ratio, sharpen_image};
use image::RgbaImage;

const TARGET_SIZE: u32 = 30;

pub(crate) struct Transform<'a> {
    task: &'a OCRTask,
    stretched_matrices: HashMap<Transformation, Vec<u32>>,
    image: RgbaImage
}

impl<'a> Transform<'a> {
    pub fn new(task: &'a OCRTask) -> Self {
        let resized_image = stretch_check_ratio(&task.image, TARGET_SIZE, TARGET_SIZE);
        Self {
            task,
            stretched_matrices: HashMap::new(),
            image: sharpen_image(&resized_image, None, None)
        }
    }
}