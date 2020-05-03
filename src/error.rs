use thiserror::Error;
use std::sync::mpsc::TrySendError;

#[derive(Error, Debug)]
pub enum KanjitomoError {
    #[error("ocr image manipulation error: {0}")]
    OCRError(image::ImageError),
}