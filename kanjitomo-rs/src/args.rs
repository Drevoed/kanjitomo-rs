use crate::{Orientation, CharacterColor};

#[derive(Debug)]
pub struct Arguments {
   pub data_dir_name: String,
   pub dictionary_dir_name: String,
   pub cache_dir_name: String,
   pub vertical: bool,
   pub orientation_target: Orientation,
   pub color_target: CharacterColor,
   pub reference_fonts: Vec<String>,
   pub reference_fonts_bold: Vec<bool>,
   pub target_size: u32,
   pub unsharp_sigma: f32,
   pub unsharp_threshold: i32,
   pub pixel_rgba_threshold: u8,
   pub ocr_halo_size: u8,

}