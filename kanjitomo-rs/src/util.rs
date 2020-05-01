use image::{GenericImage, ImageBuffer, Pixel, Rgb, RgbImage};
use std::ops::{Deref, DerefMut};

const UNSHARP_SIGMA: f32 = 4.0;
const UNSHARP_THRESHOLD: i32 = 2;

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
