use crate::atlas::TileInfo;
use crate::size::Size;


pub struct Tileset {
    pub offset: char,
    spacing: Size<i32>,
}

impl Tileset {
    pub fn new(offset: char) -> Self {
        Self {
            offset,
            spacing: Size { width: 1, height: 1 },
        }
    }

    pub fn get_offset(self) -> char {
        todo!()
    }

    pub fn provides(self) -> bool {
        todo!()
    }

    pub fn get(self, code: char) -> &'static TileInfo<'static> {
        todo!()
    }

    pub fn get_bounding_box_size(self) -> Size<i32> {
        todo!()
    }

    pub fn get_spacing(self) -> Size<i32> {
        todo!()
    }

    pub fn is_font_offset(self, offset: char) -> bool {
        todo!()
    }

    fn add_tileset(self, tileset: &Tileset) {}
}
