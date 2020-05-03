use crate::{Orientation, CharacterColor, DictionaryType};
use smart_default::SmartDefault;
use image::Rgba;

#[derive(Debug, SmartDefault)]
pub struct Arguments {
   #[default = "data"]
   pub data_dir_name: String,
   #[default = "dictionary"]
   pub dictionary_dir_name: String,
   #[default = "cache"]
   pub cache_dir_name: String,
   #[default = true]
   pub vertical: bool,
   #[default(Orientation::Auto)]
   pub orientation_target: Orientation,
   #[default(CharacterColor::Auto)]
   pub color_target: CharacterColor,
   #[default(_code = "vec![\"MS Gothic\".to_owned(), \"SimSun\".to_owned()]")]
   pub reference_fonts: Vec<String>,
   #[default(_code = "vec![false, true]")]
   pub reference_fonts_bold: Vec<bool>,
   #[default = 30]
   pub target_size: u32,
   #[default = 4.0]
   pub unsharp_sigma: f32,
   #[default = 2]
   pub unsharp_threshold: i32,
   #[default = 140]
   pub pixel_rgba_threshold: u8,
   #[default = 3]
   pub ocr_halo_size: u8,
   #[default(_code = "Rgba([255, 0, 0, 255])")]
   pub ocr_target_halo_first_color: Rgba<u8>,
   #[default(_code = "Rgba([255, 175, 175, 255])")]
   pub ocr_target_halo_last_color: Rgba<u8>,
   #[default(_code = "Rgba([100, 100, 100, 255])")]
   pub ocr_reference_halo_first_color: Rgba<u8>,
   #[default(_code = "Rgba([195, 195, 195, 255])")]
   pub ocr_reference_halo_last_color: Rgba<u8>,
   #[default = 4.0]
   pub ocr_black_pixel_score: f32,
   #[default = 4.0]
   pub ocr_white_score: f32,
   #[default(_code = "vec![-1.0, -5.0, -12.0]")]
   pub ocr_target_halo_scores: Vec<f32>,
   #[default(_code = "vec![-1.0, -4.0, -10.0]")]
   pub ocr_reference_halo_scores: Vec<f32>,
   #[default(-5.0)]
   pub ocr_connected_halo_pixels_score: f32,
   #[default = 1000.0]
   pub ocr_base_score: f32,
   #[default = 50]
   pub ocr_keep_results_lvl1: u8,
   #[default = 12]
   pub ocr_keep_results_lvl2: u8,
   #[default = 8]
   pub ocr_max_characters: u8,
   #[default = 8]
   pub ocr_threads: usize,
   #[default = 8]
   pub index_max_characters: u8,
   #[default = 1.05]
   pub default_dictionary_bias: f32,
   #[default(DictionaryType::JapaneseDefault("JMdict".to_owned()))]
   pub primary_dictionary: DictionaryType,
   #[default(DictionaryType::JapaneseNames("enamdict".to_owned()))]
   pub secondary_dictionary: DictionaryType,
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_default_fonts() {
      let args: Arguments = Default::default();
      assert_eq!(args.reference_fonts, vec!["MS Gothic", "SimSun"])
   }
}