#![allow(dead_code, unused)]
mod area;
mod error;
mod ocr;
mod traits;
mod util;
mod args;

use crate::area::Point;
use num_traits::Num;
use image::math::Rect;
use serde::{Serialize, Deserialize};
use crate::util::is_kanji;

pub fn run_ocr(point: Point) {

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
        let kanji_count = kanji.chars().fold(0, |mut i, c| {
            if is_kanji(c) {
                i += 1
            }
            i
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