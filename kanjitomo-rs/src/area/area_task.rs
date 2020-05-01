use crate::area::traits::HasRectangle;
use crate::area::{Area, Column, Point};
use crate::util::sharpen_image;
use image::{DynamicImage, ImageBuffer, Pixel, SubImage};
use std::convert::Infallible;
use std::ops::{Deref, DerefMut};

pub(crate) struct SharpenImageStep<P: Pixel> {
    task: AreaTask<P>,
}

impl<P> AreaStep<P> for SharpenImageStep<P>
where
    P: Pixel<Subpixel = u8> + 'static,
{
    fn run_impl(self) -> Result<(), Infallible> {}
}

pub(crate) enum AreaStepState {
    FindAreas,
}

pub(crate) trait AreaStep<P: Pixel>
where
    Self: Sized + 'static,
{
    //TODO remove infallible
    fn run(self) -> Result<(), Infallible> {
        self.run_impl().unwrap();
        Ok(())
    }

    fn run_impl(self) -> Result<(), Infallible>;
}

pub(crate) struct AreaStepMachine<S> {
    state: S,
}

pub struct AreaTask<P: Pixel> {
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
    pub fn new(target: ImageBuffer<P, Vec<u8>>) -> Self {
        let (width, height) = target.dimensions();
        Self {
            width,
            height,
            original_image: target,
            sharpened_image: sharpen_image(&target, None, None),
        }
    }
    fn collect_areas(&mut self) {
        self.areas = vec![];
        for column in &self.columns {
            self.areas
        }
    }

    pub(crate) fn get_area(&self, point: Point) -> Option<Area> {
        let mut min_distance = 1_000_000_u32;
        let mut closest_area: Option<Area> = None;

        for area in &self.areas {
            if area.punctuation {
                continue;
            }

            let distance = area.get_midpoint().distance(&point) as u32;
            if distance < min_distance {
                min_distance = distance;
                closest_area = Some(area.clone())
            }
        }

        match closest_area {
            None => None,
            Some(area) => {
                if min_distance > area.get_max_dim() {
                    None
                } else {
                    Some(area)
                }
            }
        }
    }

    pub(crate) fn get_sub_images(&self, point: Point) -> Vec<SubImage<ImageBuffer<P, Vec<u8>>>> {
        let sub_images = vec![];

        sub_images
    }
}
