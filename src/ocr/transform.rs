use crate::ocr::ocr_task::OCRTask;
use std::collections::HashMap;
use crate::ocr::{Transformation, TargetMatrix};
use crate::util::{sharpen_image, stretch, make_bw, build_mx_from_32_image, create_square_image, stretch_check_ratio};
use image::{RgbaImage, GenericImage, Pixel, ImageBuffer, GenericImageView, Rgba, FromColor, GrayImage};
use crate::PARAMETERS;
use crate::util::matrix_util::{move_matrix, count_bits, build_mx_halo};
use imageproc::geometric_transformations::translate;
use image::buffer::ConvertBuffer;

const TARGET_SIZE: u32 = 30;

pub(crate) struct Transform<'a> {
    task: &'a OCRTask,
    stretched_matrices: HashMap<Transformation, [u32; 32]>,
    image: GrayImage
}

impl<'a> Transform<'a> {
    pub(crate) fn new(task: &'a OCRTask) -> Self {
        let resized_image = stretch_check_ratio(&task.image, TARGET_SIZE, TARGET_SIZE);
        Self {
            task,
            stretched_matrices: HashMap::new(),
            image: sharpen_image(&resized_image, None, None)
        }
    }

    pub(crate) fn run(&self, max_translate: i32, max_stretch: i32, max_steps: i32) -> Vec<TargetMatrix> {
        let mut targets = vec![];

        for ht in -max_translate..=max_translate {
            for vt in -max_translate..=max_translate {
                for hs in -max_stretch..=max_stretch {
                    for vs in -max_stretch..=max_stretch {
                        if ht.abs() + vt.abs() + hs.abs() + vs.abs() > max_steps { continue; }
                        if (hs as f32 / 2.0).ceil() as i32 + ht.abs() > ((32 - PARAMETERS.target_size) / 2) as i32 { continue; }
                        if (vs as f32 / 2.0).ceil() as i32 + vt.abs() > ((32 - PARAMETERS.target_size) / 2) as i32 { continue; }

                        let parameters = Transformation::new(ht, vt, hs, vs);

                    }
                }
            }
        }

        targets
    }

    fn transform<I>(&mut self, image: &I, parameters: Transformation) -> TargetMatrix
    where
        I: ConvertBuffer<ImageBuffer<<I as GenericImageView>::Pixel, Vec<u8>>> + GenericImage,
        <I as GenericImageView>::Pixel: Pixel<Subpixel = u8> + FromColor<Rgba<u8>> + 'static
    {
        let mut mx = self.build_matrix(image, &parameters);
        let halo = build_mx_halo(&mut mx, PARAMETERS.ocr_halo_size - 1);
        let pixels = count_bits(&mx);

        let target = TargetMatrix::new(
            mx.clone(),
            pixels,
            halo,
            self.task.char_index.unwrap_or(0),
            parameters
        );

        target
    }

    fn build_matrix<I>(&mut self, image: &I, parameters: &Transformation) -> [u32; 32]
    where
        I: ConvertBuffer<ImageBuffer<<I as GenericImageView>::Pixel, Vec<u8>>> + GenericImage,
        <I as GenericImageView>::Pixel: Pixel<Subpixel = u8> + FromColor<Rgba<u8>> + 'static
    {
        let mut stretched = self.stretch_image(image, &parameters);
        Self::translate_matrix(&mut stretched, &parameters);
        stretched
    }

    fn stretch_image<I>(&mut self, image: &I, parameters: &Transformation) -> [u32; 32]
    where
        I: ConvertBuffer<ImageBuffer<<I as GenericImageView>::Pixel, Vec<u8>>> + GenericImage,
        <I as GenericImageView>::Pixel: Pixel<Subpixel = u8> + FromColor<Rgba<u8>> + 'static
    {
        let h_s = parameters.horizontal_stretch;
        let v_s = parameters.vertical_stretch;

        let stretch_amount = Transformation::new(0, 0, h_s, v_s);
        if let Some(stretched) = self.stretched_matrices.get(&parameters) {
            stretched.clone()
        } else {
            let new_width = PARAMETERS.target_size + h_s as u32;
            let new_height = PARAMETERS.target_size + v_s as u32;

            let grayscale = stretch(image, new_width, new_height);
            let square_grayscale = create_square_image(&grayscale, 32);
            let square_bw = make_bw(&square_grayscale, None);
            let stretched = build_mx_from_32_image(&square_bw);
            self.stretched_matrices.insert(stretch_amount, stretched.clone());
            stretched
        }
    }

    fn translate_matrix(mx: &mut [u32; 32], parameters: &Transformation) {
        move_matrix(mx, parameters.horizontal_translate, parameters.vertical_translate)
    }
}