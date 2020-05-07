use crate::area::{Area, Column, Point};
use crate::util::{sharpen_image, crop};
use image::{DynamicImage, ImageBuffer, Pixel, SubImage, GenericImage, Luma, GrayImage};
use std::convert::Infallible;
use std::ops::{Deref, DerefMut};
use image::math::Rect;
use crate::traits::{Step, Task};
use crate::error::KanjitomoError;


#[derive(Debug, Clone)]
pub struct AreaTask<P>
where
    P: Pixel<Subpixel = u8> + 'static
{
    width: u32,
    height: u32,
    original_image: ImageBuffer<P, Vec<u8>>,
    sharpened_image: ImageBuffer<P, Vec<u8>>,
    inverted: Vec<Vec<bool>>,
    binary_image: Vec<Vec<bool>>,
    background_image: Vec<Vec<bool>>,
    border_pixels: Vec<Vec<bool>>,
    areas: Vec<Area>,
    columns: Vec<Column>,
    vertical_columns: Vec<Column>,
    horizontal_columns: Vec<Column>,
}

impl<P> AreaTask<P>
where
    P: Pixel<Subpixel = u8> + 'static,
{
    // pub fn new(target: ImageBuffer<P, Vec<u8>>) -> Self {
    //     let (width, height) = target.dimensions();
    //     let sharpened_image = sharpen_image(&target, None, None);
    //     Self {
    //         width,
    //         height,
    //         original_image: target,
    //         sharpened_image,
    //         ..Default::default()
    //     }
    // }

    pub(crate) fn get_subimages(&self, areas: Vec<Rect>) -> Vec<SubImage<ImageBuffer<P, Vec<u8>>>> {
        let mut subimages = vec![];

        for area in areas {

        }

        subimages
    }
    // fn collect_areas(&mut self) {
    //     self.areas = vec![];
    //     for column in &self.columns {
    //         self.areas
    //     }
    // }
    //
    // pub(crate) fn get_area(&self, point: Point) -> Option<Area> {
    //     let mut min_distance = 1_000_000_u32;
    //     let mut closest_area: Option<Area> = None;
    //
    //     for area in &self.areas {
    //         if area.punctuation {
    //             continue;
    //         }
    //
    //         let distance = area.get_midpoint().distance(&point) as u32;
    //         if distance < min_distance {
    //             min_distance = distance;
    //             closest_area = Some(area.clone())
    //         }
    //     }
    //
    //     match closest_area {
    //         None => None,
    //         Some(area) => {
    //             if min_distance > area.get_max_dim() {
    //                 None
    //             } else {
    //                 Some(area)
    //             }
    //         }
    //     }
    // }

    pub(crate) fn get_sub_images(&self, point: Point) -> Vec<SubImage<ImageBuffer<P, Vec<u8>>>> {
        let sub_images = vec![];

        sub_images
    }
}

pub(crate) struct AreaDetector<'a, P>
where
    P: Default + Pixel<Subpixel = u8> + 'static
{
    task: &'a mut AreaTask<P>
}

impl<'a, P> AreaDetector<'a, P>
where
    P: Default + Pixel<Subpixel = u8> + 'static
{
    pub(crate) fn new(task: &'a mut AreaTask<P>) -> Self {
        Self { task }
    }


}

pub(crate) enum AreaTaskStep<'a, P>
where
    P: Default + Pixel<Subpixel = u8> + 'static
{
    SharpenImage(&'a mut AreaTask<P>),
    CreateBinaryImage(&'a mut AreaTask<P>),
    InvertImage()
}
