use crate::point::Point;
use crate::size::Size;
use crate::rectangle::Rectangle;
use crate::tileset::Tileset;

use bitmaps::Bitmap;
// use std::ops::{Shl, Shr};


const BITMAP_SIZE: usize = 16;


pub struct TexCoords {
    tu1: f32,
    tv1: f32,
    tu2: f32,
    tv2: f32,
}


pub enum TileAlignment {
    Unknown,
    Center,
    DeadCenter,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}


pub struct TileInfo<'a> {
    tileset: Tileset,
    texture: AtlasTexture,
    bitmap: &'a mut Bitmap<BITMAP_SIZE>,
    useful_space: Rectangle<i32>,
    total_space: Rectangle<i32>,
    texture_coords: TexCoords,
    offset: Point<i32>,
    spacing: Size<i32>,
    alignment: TileAlignment,
    is_animated: bool,
}


pub struct AtlasTexture {
    initial_size: Size<i32>,
}

impl AtlasTexture {
    pub fn new(initial_size: Size<i32>) -> Self {
        Self { initial_size }
    }

    pub fn add(tile: &TileInfo) {
        todo!()
    }

    pub fn remove(tile: &TileInfo) {
        todo!()
    }

    pub fn defragment() {
        todo!()
    }

    pub fn clean_up() {
        todo!()
    }

    pub fn clear() {
        todo!()
    }

    pub fn apply_texture_filter() {
        todo!()
    }

    fn try_grow() {
        todo!()
    }

    fn calculate_tex_coords() -> TexCoords {
        todo!()
    }
}


pub struct Atlas {
    textures: Vec<AtlasTexture>,
}

impl Atlas {
    pub fn add(tile: &TileInfo) {
        todo!()
    }

    pub fn remove(tile: &TileInfo) {
        todo!()
    }

    pub fn defragment() {
        todo!()
    }

    pub fn clean_up() {
        todo!()
    }

    pub fn clear() {
        todo!()
    }

    pub fn apply_texture_filter() {
        todo!()
    }
}
