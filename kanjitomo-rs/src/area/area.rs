use crate::area::column::Column;
use crate::area::Point;
use crate::traits::HasRectangle;
use image::imageops::ColorMap;
use image::math::Rect;
use image::ColorType;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::traits::HasRectangle;

#[derive(Debug, Clone)]
pub struct Area {
    rect: Rect,
    pixels: u32,
    pub(crate) punctuation: bool,
    changed: bool,
    pub splitted: bool,
    pub min_rgb: u32,
    pub column: Weak<RefCell<Column>>,
    remove: bool,
    source_areas: Vec<Area>,
}

impl HasRectangle for Area {
    fn get_rectangle(&self) -> Rect {
        self.rect
    }

    #[inline(always)]
    fn get_midpoint(&self) -> Point {
        Point {
            x: self.rect.x + self.rect.width / 2,
            y: self.rect.y + self.rect.height / 2,
        }
    }
}

impl Area {
    pub fn new(rect: Rect, pixels: u32) -> Self {
        Self {
            rect,
            pixels,
            punctuation: false,
            changed: false,
            splitted: false,
            min_rgb: Default::default(),
            column: Weak::new(),
            remove: false,
            source_areas: vec![],
        }
    }

    pub fn get_size(&self) -> u32 {
        self.rect.width * self.rect.height
    }

    pub fn height(&self) -> u32 {
        self.rect.height
    }

    pub fn width(&self) -> u32 {
        self.rect.width
    }

    pub fn get_y(&self) -> u32 {
        self.rect.y
    }

    pub fn get_x(&self) -> u32 {
        self.rect.x
    }

    pub fn get_max_x(&self) -> u32 {
        self.rect.x + self.rect.width - 1
    }

    pub fn get_max_y(&self) -> u32 {
        self.rect.y + self.rect.height - 1
    }

    #[inline(always)]
    fn get_ratio(&self) -> f32 {
        let r1: f32 = self.rect.width as f32 / self.rect.height as f32;
        let r2: f32 = self.rect.height as f32 / self.rect.width as f32;

        r1.min(r2)
    }

    pub(crate) fn get_max_dim(&self) -> u32 {
        self.rect.width.max(self.rect.height)
    }

    pub(crate) fn get_min_dim(&self) -> u32 {
        self.rect.width.min(self.rect.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
