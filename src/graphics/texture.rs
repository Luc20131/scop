use std::path::Path;

use crate::my_lib::bmp_parser::image_loader;

pub struct Texture {
    id: u32,
}

impl Texture {
    pub fn new(path: &Path) -> Self {
        image_loader(&path);
        Self { id: 0 }
    }
}
