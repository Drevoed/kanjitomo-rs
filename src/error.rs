use thiserror::Error;

#[derive(Error, Debug)]
pub enum KanjitomoError {
    #[error("Ocr image manipulation error: {0}")]
    OCRError(image::ImageError),
    #[error("Scale error: min_source_value - {min_source_value:?} larger than max_source_value - {max_source_value:?}")]
    ScaleError {
        min_source_value: f32,
        max_source_value: f32
    },
    #[error("Something unexpected happened: {0}")]
    Custom(String)
}