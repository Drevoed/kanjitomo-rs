#![allow(dead_code, unused)]
mod area;
mod error;
mod ocr;
mod traits;
mod util;
mod parameters;

use crate::area::Point;
use num_traits::Num;
use serde::{Serialize, Deserialize};
use parameters::Parameters;
use lazy_static::lazy_static;
use crate::util::is_kanji;
use crate::ocr::OCRManager;

lazy_static! {
    pub static ref PARAMETERS: Parameters = Default::default();
}

pub struct KanjiTomo {
    ocr: OCRManager,
}

impl KanjiTomo {
    pub fn new() {

    }
    pub fn run_ocr(&mut self, point: Point) {

    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Orientation {
    Auto,
    Vertical,
    Horizontal
}

#[derive(Debug, Eq, PartialEq)]
pub enum CharacterColor {
    Auto,
    BlackOnWhite,
    WhiteOnBlack
}

#[derive(Debug, Eq, PartialEq)]
pub enum DictionaryType {
    JapaneseDefault(String),
    JapaneseNames(String),
    Chinese(String)
}

#[derive(Debug)]
pub struct OCRResult {
    pub best_matching_characters: Vec<String>,
    pub characters: Vec<IdentifiedCharacter<u32>>,
    pub search_string: String,
}

#[derive(Debug)]
pub struct IdentifiedCharacter<N: Num> {
    pub reference_characters: String,
    pub scores: Vec<N>,
    pub location: Rect,
}

impl<N> IdentifiedCharacter<N>
where
    N: Num
{
    pub fn new(matched_characters: String, location: Rect, scores: Vec<N>) -> Self {
        Self {
            reference_characters: matched_characters,
            location,
            scores
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Word {
    pub kanji: String,
    pub kana: String,
    pub description: String,
    pub common: bool,
    pub name: bool,
    pub kanji_count: u32,
}

impl Word {
    pub fn new(kanji: String, kana: String, description: String, name: bool) -> Self {
        let kanji_count = kanji.chars().fold(0, |i, c| {
            if is_kanji(c) {
                i + 1
            } else {
                i
            }
        });

        let common = description.contains("(P)");
        Self {
            kanji,
            kana,
            description,
            name,
            common,
            kanji_count
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default, Serialize)]
pub struct Rect {
    /// The x coordinate of the top left corner.
    pub x: u32,
    /// The y coordinate of the top left corner.
    pub y: u32,
    /// The rectangle's width.
    pub width: u32,
    /// The rectangle's height.
    pub height: u32,
}

#[cfg(test)]
mod tests {
    use crate::{PARAMETERS, Word};
    use crate::parameters::Parameters;

    #[test]
    fn test_static_parameters() {
        assert_eq!(*PARAMETERS, Parameters::default())
    }

    #[test]
    fn test_kanji_count() {
        let word = Word::new("腹切り".to_owned(), "".to_owned(), "".to_owned(), false);

        assert_eq!(2, word.kanji_count)
    }
}