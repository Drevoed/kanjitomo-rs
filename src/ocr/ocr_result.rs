use crate::ocr::{ReferenceMatrix, TargetMatrix};

#[derive(Default, Clone, Debug)]
pub(crate) struct OCRResult {
    target: TargetMatrix,
    pub(crate) reference: ReferenceMatrix,
    black_pixels: u32,
    white_pixels: u32,
    target_halo_pixels: u32,
    reference_halo_pixels: u32,
    pub(crate) score: u32,
    pub(crate) avg_score: f32,
    refined_alignment: bool,
}

impl OCRResult {
    pub(crate) fn new(target: TargetMatrix, reference: ReferenceMatrix) -> Self {
        Self {
            target,
            reference,
            refined_alignment: false,
            ..Default::default()
        }
    }

    pub(crate) fn get_character(&self) -> char {
        self.reference.character
    }
}
