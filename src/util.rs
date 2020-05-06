use imageproc::rect::Rect;
use num_traits::Zero;
use image::{GenericImage, ImageBuffer, Pixel, Primitive, Rgb, RgbImage, Rgba, RgbaImage, SubImage, GrayImage, Luma, GenericImageView, FromColor};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use bit::BitIndex;
use palette::Srgb;
use palette::named;
use palette::encoding::pixel::Pixel as _;

pub(crate) use char_util::*;
use image::imageops::{FilterType, dither, BiLevel};
use crate::error::KanjitomoError;
use image::math::utils::clamp;
use imageproc::drawing::draw_filled_rect_mut;
use crate::PARAMETERS;
use nalgebra::{MatrixN, dimension::U32, DMatrix};
use crate::util::matrix_util::is_bit_set;
use image::buffer::ConvertBuffer;

pub(crate) fn contains_pixel(rgb: u32, black_threshold: u8) -> bool {
    let red = ((rgb & 0x00ff0000) >> 16) < black_threshold as u32;
    let green = ((rgb & 0x0000ff00) >> 8) < black_threshold as u32;
    let blue = (rgb & 0x000000ff) < black_threshold as u32;

    (red && green) || (green && blue) || (red && blue)
}

pub(crate) fn build_scaled_image<I, P>(image: &I, scale: u32) -> ImageBuffer<P, Vec<u8>>
where
    I: GenericImage<Pixel = P>,
    P: Pixel<Subpixel = u8> + 'static,
{
    let mut target = ImageBuffer::new(image.width() * scale, image.height() * scale);

    for y in 0..image.height() {
        for x in 0..image.width() {
            let rgb = image.get_pixel(x, y);
            for ty in y * scale..(y + 1) * scale {
                for tx in x * scale..(x + 1) * scale {
                    target.put_pixel(tx, ty, rgb.clone())
                }
            }
        }
    }

    target
}

pub(crate) fn create_empty_copy<I, P>(image: &I) -> ImageBuffer<P, Vec<u8>>
where
    I: GenericImage<Pixel = P>,
    P: Pixel<Subpixel = u8> + 'static,
{
    ImageBuffer::new(image.width(), image.height())
}

pub(crate) fn sharpen_image<I, P>(
    img: &I,
    sigma: Option<f32>,
    threshold: Option<i32>,
) -> ImageBuffer<P, Vec<u8>>
where
    I: GenericImage<Pixel = P>,
    P: Pixel<Subpixel = u8> + 'static,
{
    image::imageops::unsharpen(
        img,
        sigma.unwrap_or(PARAMETERS.unsharp_sigma),
        threshold.unwrap_or(PARAMETERS.unsharp_threshold),
    )
}

pub(crate) fn crop<I, P>(img: &I, rect: image::math::Rect) -> SubImage<&I>
where
    I: GenericImage<Pixel = P>,
    P: Pixel<Subpixel = u8> + 'static,
{
    image::imageops::crop_imm(img, rect.x, rect.y, rect.width, rect.height)
}

// pub(crate) fn image_from_matrix(mx: ImgRef<bool>) -> RgbaImage {
//     let width = mx.width() as u32;
//     let height = mx.height() as u32;
//
//     let mut b_image = ImageBuffer::new(width, height);
//     for x in 0..width {
//         for y in 0..height {
//             if !mx[(x, y)] {
//                 b_image.put_pixel(x, y, LUMAColors::WHITE)
//             }
//         }
//     }
//
//     b_image
// }

// pub(crate) fn matrix_from_image(img: &RgbaImage) -> ImgVec<bool>
// {
//     let (width, height) = img.dimensions();
//     let mut mx = ImgVec::new(vec![false; width as usize * height as usize], width as usize, height as usize);
//
//     for x in 0..width {
//         for y in 0..height {
//             let pixel = img.get_pixel(x, y);
//             if pixel == &LUMAColors::BLACK {
//                 mx[(x, y)] = true;
//             }
//         }
//     }
//
//     mx
// }

// build 32x32 matrix from 32x32 image
pub(crate) fn build_mx_from_32_image(image: &GrayImage) -> [u32; 32]
{
    let mut mx = [0u32; 32];

    for y in 0..32 {
        for x in 0..32 {
            if image.get_pixel(x, y) == &Luma(*named::WHITE.as_raw()) {
                mx[y as usize].set_bit(x as usize, true);
            }
        }
    }

    mx
}

pub(crate) fn make_bw<I>(img: &I, black_threshold: Option<u8>) -> GrayImage
where
    I: GenericImage,
    <I as GenericImageView>::Pixel: Pixel<Subpixel = u8> + 'static
{
    let mut grayscale = image::imageops::grayscale(img);

    dither(&mut grayscale, &BiLevel);

    grayscale
}

pub(crate) fn build_image_from_bit_mx(mx: &[u32; 32]) -> GrayImage
{
    let mut image = ImageBuffer::new(32 ,32);

    for (x, y, p) in image.enumerate_pixels_mut() {
        if matrix_util::is_bit_set(x, y, mx) {
            *p = Luma(*named::BLACK.as_raw());
        } else {
            *p = Luma(*named::WHITE.as_raw())
        }
    }

    image
}

pub(crate) fn build_bit_mx(image: &GrayImage) -> Vec<u32>
{
    let (width, height) = image.dimensions();
    let mut mx = vec![0_u32; height as usize];

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            if pixel == &Luma(*named::BLACK.as_raw()) {
                mx[y as usize].set_bit(x as usize, true);
            }
        }
    }

    mx
}

pub(crate) fn stretch<I, P>(img: &I, width: u32, height: u32) -> ImageBuffer<P, Vec<u8>>
where
    I: GenericImage<Pixel = P>,
    P: Pixel<Subpixel = u8> + 'static,
{
    let scaled_image = image::imageops::resize(img, width, height, FilterType::Lanczos3);

    scaled_image
}

pub(crate) fn stretch_check_ratio<I>(img: &I, target_size: u32, final_size: u32) -> ImageBuffer<I::Pixel, Vec<u8>>
where
    I: ConvertBuffer<ImageBuffer<<I as GenericImageView>::Pixel, Vec<u8>>> + GenericImage,
    <I as GenericImageView>::Pixel: Pixel<Subpixel = u8> + FromColor<Rgba<u8>> + 'static
{
    let (width, height) = img.dimensions();
    let mut ratio = (width / height) as f32;
    if ratio > 1.0_f32 {
        ratio = 1_f32 / ratio;
    }

    let mut target_height = target_size;
    let mut target_width = target_size;

    let target_min_dim = scale(ratio, 0.1, 0.4, 8 as f32, target_size as f32).unwrap();

    if width > height {
        target_height = target_min_dim
    } else {
        target_width = target_min_dim;
    }

    let stretched = stretch(img, target_width, target_height);

    create_square_image(&stretched, final_size)
}

pub(crate) fn create_square_image<I>(source_img: &I, size: u32) -> ImageBuffer<I::Pixel, Vec<u8>>
where
    I: ConvertBuffer<ImageBuffer<<I as GenericImageView>::Pixel, Vec<u8>>> + GenericImage,
    <I as GenericImageView>::Pixel: Pixel<Subpixel = u8> + FromColor<Rgba<u8>> + 'static
{
    let (width, height) = source_img.dimensions();

    let mut block_image = create_white_image(size, size);

    let delta_x = (size - width) / 2;
    let delta_y = (size - height) / 2;

    for y in 0..height {
        let target_y = y + delta_y;
        if target_y >= size {
            continue;
        }
        for x in 0..width {
            let target_x = x + delta_x;
            if target_x >= size {
                continue;
            }
            let pixel = source_img.get_pixel(x, y);
            block_image.put_pixel(x, y, pixel)
        }
    }

    block_image
}

pub(crate) fn create_white_image<P>(width: u32, height: u32) -> ImageBuffer<P, Vec<u8>>
where
    P: Pixel<Subpixel = u8> + FromColor<Rgba<u8>> + 'static
{
    let mut image = ImageBuffer::new(width, height);
    draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(width, height), Rgba(*named::WHITE.as_raw()));

    image.convert()
}

pub(crate) fn scale(mut source_value: f32, min_source_value: f32, max_source_value: f32, target_1: f32, target_2: f32) -> Result<u32, KanjitomoError> {
    if min_source_value > max_source_value {
        return Err(KanjitomoError::ScaleError {
            min_source_value,
            max_source_value
        });
    }

    let source_value = clamp(source_value, min_source_value, max_source_value);
    let scale = (source_value - min_source_value) / (max_source_value - source_value);

    let res = target_1 * (1_f32 - scale) + target_2 * scale;

    Ok(res.round() as u32)
}

pub(crate) mod matrix_util {

    pub(crate) fn move_matrix(mx: &mut [u32; 32], h: i32, v: i32) {
        for y in 0_i32..mx.len() as i32 {
            let new_y = y + v;
            if new_y < 0 || new_y > 31 { continue ;}

            if h >= 0 {
                mx[new_y as usize] = mx[y as usize] >> h as u32
            } else {
                mx[new_y as usize] = mx[y as usize] << (-1 * h) as u32;
            }
        }
    }

    pub(crate) fn is_bit_set(x: u32, y: u32, mx: &[u32]) -> bool {
        if x >= 32 || y >= 32 {
            false
        } else {
            let row = mx[y as usize];
            (row & (1 << (31 - x))) != 0
        }
    }
}

mod char_util {
    #[inline(always)]
    pub(crate) fn is_hiragana(c: char) -> bool {
        if c == '|' {
            true
        } else {
            let c = c as u32;
            c >= 0x3040 && c <= 0x309F
        }
    }

    #[inline(always)]
    pub(crate) fn is_katakana(c: char) -> bool {
        if c == '|' {
            true
        } else {
            let c = c as u32;
            c >= 0x30A0 && c <= 0x30FF
        }
    }

    #[inline(always)]
    pub(crate) fn is_kana(c: char) -> bool {
        is_hiragana(c) || is_katakana(c)
    }

    #[inline(always)]
    pub(crate) fn is_kanji(c: char) -> bool {
        let c = c as u32;
        if c == 0x3005 {
            true
        } else {
            c >= 0x4E00 && c <= 0x9FAF
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use image::{open, DynamicImage};
    use std::path::PathBuf;
    use crate::util::char_util::{is_kanji, is_hiragana, is_katakana, is_kana};

    pub(crate) const PATH: &str = "C:\\Users\\vetro\\RustProjects\\kanjitomo-rs\\images_test\\image1.jpg";

    fn get_image() -> DynamicImage {
        println!("opening image...");
        let image = open(&PATH).unwrap();
        println!("opened image");
        image
    }

    #[test]
    fn test_sharpen() {
        let image = get_image();
        log::debug!("sharpen image...");
        let mut sharpened = sharpen_image(&image, None, None);
        log::debug!("sharpened image");
        log::debug!("bw image...");
        let bw = make_bw(&sharpened, None);
        log::debug!("bwed image");
        log::debug!("saving image...");
        sharpened.save(&PATH).unwrap();
        log::debug!("saved image");
    }

    #[test]
    fn test_mx_output() {
        pretty_env_logger::try_init().unwrap_or(());
        let mut image = get_image().into_rgba();
        let image = sharpen_image(&image, None, None);
        let image = make_bw(&image, None);
        let mx = build_bit_mx(&image);

        let image = build_image_from_bit_mx(mx.as_slice());
        image.save(&PATH).unwrap();
    }

    #[test]
    fn test_is_kanji() {
        assert_eq!(true, is_kanji('漢'));
        assert_eq!(true, is_kanji('字'));
        assert_eq!(true, is_kanji('名'))
    }

    #[test]
    fn test_is_hiragana() {
        assert_eq!(true, is_hiragana('ぬ'));
        assert_eq!(true, is_hiragana('へ'));
        assert_eq!(true, is_hiragana('よ'));
    }

    #[test]
    fn test_is_katakana() {
        assert_eq!(true, is_katakana('ウ'));
        assert_eq!(true, is_katakana('チ'));
        assert_eq!(true, is_katakana('ハ'));
    }

    #[test]
    fn test_is_kana() {
        assert_eq!(true, is_kana('チ'));
        assert_eq!(true, is_kana('へ'));
    }

    //#[test]
    fn test_rect_image() {
        let mut image: RgbaImage = ImageBuffer::new(128, 128);
        draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(128, 128), Rgba([255, 0, 255, 255]));
        image.save(&PATH).unwrap()
    }
}
