use image::GrayImage;

pub(crate) struct OCRTask {
    // Target grayscale sub-image around single character against which OCR is run
    image: GrayImage,
    char_index: Option<u32>,
}
