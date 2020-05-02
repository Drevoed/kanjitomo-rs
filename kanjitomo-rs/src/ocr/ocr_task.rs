use image::{GrayImage, RgbaImage, GenericImage, Pixel};
use super::OCRResult;

#[derive(Debug, Clone)]
pub(crate) struct OCRTask
{
    pub(crate) image: RgbaImage,
    char_index: Option<u32>,
    results: Vec<OCRResult>,
    column_changed: bool,
}

impl OCRTask
{
    pub(crate) fn new(image: RgbaImage) -> Self {
        Self {
            image,
            char_index: None,
            results: vec![],
            column_changed: false
        }
    }

    pub(crate) fn get_character(&self) -> Option<char> {
        if self.results.len() > 0 {
            Some(self.results[0].get_character())
        } else {
            None
        }
    }

    pub(crate) fn get_result_string(&self) -> String {
        let mut string = String::new();

        for result in &self.results {
            string.push(result.reference.character)
        }

        string
    }
}
