use crate::ocr::{TargetMatrix, ReferenceMatrix};

#[derive(Default, Clone, Debug)]
pub(crate) struct OCRResult {
    target: TargetMatrix,
    reference: ReferenceMatrix,
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
}
