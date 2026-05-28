use crate::my_lib::bmp_parser::image_loader;

pub struct Texture {
    id: u32,
}

impl Texture {
    pub fn new(path: &str) -> Self {
        image_loader(path);
        Self { id: 0 }
    }
}
