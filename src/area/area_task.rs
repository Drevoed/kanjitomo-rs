use crate::area::{Area, Column, Point};
use crate::util::{sharpen_image, crop, make_bw, matrix_from_image};
use image::{DynamicImage, ImageBuffer, Pixel, SubImage, GenericImage, Luma, GrayImage, FromColor, Rgba};
use std::convert::Infallible;
use std::ops::{Deref, DerefMut};
use image::math::Rect;
use crate::traits::{Step, Task};
use crate::error::KanjitomoError;
use crate::{PARAMETERS, CharacterColor};
use nalgebra::base::DMatrix;


#[derive(Debug, Clone)]
pub struct AreaTask<P>
where
    P: Pixel<Subpixel = u8> + 'static
{
    width: u32,
    height: u32,
    original_image: ImageBuffer<P, Vec<u8>>,
    sharpened_image: Option<ImageBuffer<P, Vec<u8>>>,
    inverted: Option<DMatrix<bool>>,
    binary_image: Option<DMatrix<bool>>,
    background_image: Option<DMatrix<bool>>,
    border_pixels: Option<DMatrix<bool>>,
    areas: Option<Vec<Area>>,
    columns: Option<Vec<Column>>,
    vertical_columns: Option<Vec<Column>>,
    horizontal_columns: Option<Vec<Column>>,
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
            sharpened_image: None,
            inverted: None,
            binary_image: None,
            background_image: None,
            border_pixels: None,
            areas: None,
            columns: None,
            vertical_columns: None,
            horizontal_columns: None,
        }
    }

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

pub(crate) struct AreaDetector<P>
where
    P: Pixel<Subpixel = u8> + FromColor<Rgba<u8>> + 'static
{
    task: AreaTask<P>,
    step: AreaTaskStep
}

impl<'a, P> AreaDetector<P>
where
    P: Pixel<Subpixel = u8> + FromColor<Rgba<u8>> + 'static
{
    pub(crate) fn new(task: AreaTask<P>) -> Self {
        Self { task, step: AreaTaskStep::SharpenImage }
    }

    pub(crate) fn run(&mut self) -> AreaTask<P> {
        use AreaTaskStep::*;

        loop {
            let next = match self.step {
                SharpenImage => {
                    self.task.sharpened_image = Some(sharpen_image(&self.task.original_image, None, None));
                    CreateBinaryImage
                },
                CreateBinaryImage => {
                    let sharpened = self.task.sharpened_image.as_ref().expect("For some reason binary image was created before sharpening!");
                    let bw = make_bw(sharpened, None);
                    self.task.binary_image = Some(matrix_from_image(&bw));
                    InvertImage(InvertImageData::new())
                },
                InvertImage(ref mut inv_step) => {
                    inv_step.run(&mut self.task);
                    FindAreas
                },
                _ => FindAreas
            };
            self.step = next;
        }
        let mut task;
        std::mem::replace(&mut task, self.task);
        task
    }
}

pub(crate) enum AreaTaskStep
{
    SharpenImage,
    CreateBinaryImage,
    InvertImage(InvertImageData),
    FindAreas
}

pub(crate) struct InvertImageData
{
    width: u32,
    height: u32,
    visited: Option<DMatrix<bool>>,
    invert: Option<DMatrix<bool>>,
    neighbours_inverted: Option<DMatrix<u32>>
}

impl InvertImageData {
    const BLOCK_SIZE: u32 = 15;

    fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            visited: None,
            invert: None,
            neighbours_inverted: None,
        }
    }

    fn run<P>(&mut self, task: &mut AreaTask<P>)
    where
        P: Pixel<Subpixel = u8> + FromColor<Rgba<u8>> + 'static
    {
        if PARAMETERS.fixed_black_level {
            ()
        } else {
            match PARAMETERS.color_target {
                CharacterColor::BlackOnWhite => (),
                CharacterColor::Auto => {
                    self.detect_b_on_w(task)
                },
                CharacterColor::WhiteOnBlack => ()
            }
        }
    }

    fn detect_b_on_w<P>(&mut self, task: &mut AreaTask<P>)
    where
        P: Pixel<Subpixel = u8> + FromColor<Rgba<u8>> + 'static
    {
        self.width = (task.width as f32 / Self::BLOCK_SIZE as f32).ceil() as u32;
        self.height = (task.height as f32 / Self::BLOCK_SIZE as f32).ceil() as u32;
        let bool_mx = DMatrix::from_row_slice(self.height as usize, self.width as usize, &vec![vec![false; self.width as usize]; self.height as usize].into_iter().flatten().collect::<Vec<bool>>()[..]);

        let visited = bool_mx.clone();
        let invert = bool_mx;
        let neighbours_inverted = DMatrix::from_row_slice(self.height as usize, self.width as usize, &vec![vec![0; self.width as usize]; self.height as usize].into_iter().flatten().collect::<Vec<u32>>()[..]);
        task.border_pixels = Some(DMatrix::from_row_slice(task.height as usize, task.width as usize, &vec![vec![false; task.width as usize]; task.height as usize].into_iter().flatten().collect::<Vec<bool>>()[..]));

        for x in 0..self.width {

        }
    }

    fn check_block(&mut self, x: u32, y: u32) {
        let mut marked: Vec<Block> = vec![];

        let black_blocks = 0;

        
    }
}

struct Block {
    x: u32,
    y: u32,
}
