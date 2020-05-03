use imageproc::rect::Rect;
use num_traits::Zero;
use image::{
    GenericImage, ImageBuffer, Pixel, Primitive, Rgb, RgbImage, Rgba, RgbaImage, SubImage,
};
use imgref::{ImgRef, ImgVec};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub(crate) use char_util::*;
use image::imageops::FilterType;
use crate::error::KanjitomoError;
use image::math::utils::clamp;
use imageproc::drawing::draw_filled_rect_mut;

const UNSHARP_SIGMA: f32 = 4.0;
const UNSHARP_THRESHOLD: i32 = 2;

pub(crate) struct RGBAColors;

impl RGBAColors {
    const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);
    const BLACK: Rgba<u8> = Rgba([0, 0, 0, 255]);
}

pub(crate) fn contains_pixel(rgb: u32, black_threshold: u32) -> bool {
    let red = ((rgb & 0x00ff0000) >> 16) < black_threshold;
    let green = ((rgb & 0x0000ff00) >> 8) < black_threshold;
    let blue = (rgb & 0x000000ff) < black_threshold;

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

pub(crate) fn create_copy<I, P>(image: &I) -> ImageBuffer<P, Vec<u8>>
where
    I: GenericImage<Pixel = P>,
    P: Pixel<Subpixel = u8> + 'static,
{
    let (width, height) = image.dimensions();
    let mut out = ImageBuffer::new(width, height);

    for (x, y, p) in image.pixels() {
        out.put_pixel(x, y, p)
    }

    out
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
        sigma.unwrap_or(UNSHARP_SIGMA),
        threshold.unwrap_or(UNSHARP_THRESHOLD),
    )
}

pub(crate) fn crop<I, P>(img: &I, rect: image::math::Rect) -> SubImage<&I>
where
    I: GenericImage<Pixel = P>,
    P: Pixel<Subpixel = u8> + 'static,
{
    image::imageops::crop_imm(img, rect.x, rect.y, rect.width, rect.height)
}

pub(crate) fn image_from_matrix(mx: ImgRef<bool>) -> RgbaImage {
    let width = mx.width() as u32;
    let height = mx.height() as u32;

    let mut b_image = ImageBuffer::new(width, height);
    for x in 0..width {
        for y in 0..height {
            if !mx[(x, y)] {
                b_image.put_pixel(x, y, RGBAColors::WHITE)
            }
        }
    }

    b_image
}

pub(crate) fn matrix_from_image(img: &RgbaImage) -> ImgVec<bool>
{
    let (width, height) = img.dimensions();
    let mut mx = ImgVec::new(vec![false; width as usize * height as usize], width as usize, height as usize);

    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            if pixel == &RGBAColors::BLACK {
                mx[(x, y)] = true;
            }
        }
    }

    mx
}

pub(crate) fn build_matrix_32<I>(image: &I) -> [u8; 32]
where
    I: GenericImage<Pixel = Rgba<u8>>,
{
    let mut mx = [0u8; 32];

    for y in 0..32 {
        for x in 0..32 {
            if image.get_pixel(x, y) == RGBAColors::WHITE {
                mx[y as usize] |= 1;
            }
            if x < 31 {
                mx[y as usize] <<= 1;
            }
        }
    }

    mx
}

pub(crate) fn make_bw(image: &RgbaImage, black_threshold: u32) -> RgbaImage
{
    let mut bw_image = create_empty_copy(image);

    for (x, y, p) in image.enumerate_pixels() {
        let pixel = contains_pixel(u32::from_le_bytes(p.0), black_threshold);
        if pixel {
            bw_image.put_pixel(x, y, RGBAColors::BLACK);
        } else {
            bw_image.put_pixel(x, y, RGBAColors::WHITE);
        }
    }

    bw_image
}

pub(crate) fn build_image(mx: &Vec<u32>) -> RgbaImage
{
    let mut image = ImageBuffer::new(32 ,32);

    for x in 0..32 {
        for y in 0..32 {
            if matrix_util::is_bit_set(x, y, mx) {
                image.put_pixel(x, y, RGBAColors::BLACK)
            } else {
                image.put_pixel(x, y, RGBAColors::WHITE)
            }
        }
    }

    image
}

pub(crate) fn build_matrix(image: &RgbaImage) -> Vec<u32>
{
    let (width, height) = image.dimensions();
    let mut mx = vec![0_u32; height as usize];

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            if pixel == &RGBAColors::BLACK {
                mx[y as usize] |= 1;
            }
            if x < width - 1 {
                mx[y as usize] <<= 1;
            }
        }
    }

    mx.into()
}

pub(crate) fn stretch<I, P>(img: &I, width: u32, height: u32) -> ImageBuffer<P, Vec<u8>>
where
    I: GenericImage<Pixel = P>,
    P: Pixel<Subpixel = u8> + 'static,
{
    let scaled_image = image::imageops::resize(img, width, height, FilterType::Lanczos3);

    scaled_image
}

pub(crate) fn stretch_check_ratio(img: &RgbaImage, target_size: u32, final_size: u32) -> RgbaImage
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

    let stretched: RgbaImage = stretch(img, target_width, target_height);

    create_square_image(&stretched, final_size)
}

pub(crate) fn create_square_image(source_img: &RgbaImage, size: u32) -> RgbaImage
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
            block_image.put_pixel(x, y, *pixel)
        }
    }

    block_image
}

pub(crate) fn create_white_image(width: u32, height: u32) -> RgbaImage {
    let mut image = ImageBuffer::new(width, height);
    draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(width, height), RGBAColors::WHITE);

    image
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

mod matrix_util {
    use imgref::ImgRef;

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

    //#[test]
    fn test_sharpen() {
        let image = get_image();
        log::debug!("sharpen image...");
        let mut sharpened = sharpen_image(&image, None, None);
        log::debug!("sharpened image");
        log::debug!("bw image...");
        sharpened = make_bw(&sharpened, 135);
        log::debug!("bwed image");
        log::debug!("saving image...");
        sharpened.save(&PATH).unwrap();
        log::debug!("saved image");
    }

    #[test]
    fn test_mx_output() {
        pretty_env_logger::try_init().unwrap_or(());
        let mut image = get_image().into_rgba();
        image = sharpen_image(&image, None, None);
        image = make_bw(&image, 135);
        let mx = build_matrix(&image);

        let image = build_image(mx.as_ref());
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
