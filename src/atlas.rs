use bitmaps::Bitmap;


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


pub struct Tileset {

}

pub struct AtlasTexture {

}

impl AtlasTexture {
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

pub struct TileInfo<'a> {
    tileset: Tileset,
    texture: AtlasTexture,
    bitmap: &'a mut Bitmap<BITMAP_SIZE>,
    texture_coords: TexCoords,
    alignment: TileAlignment,
}
