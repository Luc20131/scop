use crate::parsers::bmp_parser::image_loader;
use std::{ffi::OsString, path::Path};

#[allow(dead_code)]
pub struct Texture {
    id: u32,
    name: OsString,
}

impl Texture {
    pub fn new(path: &Path) -> Self {
        image_loader(path);

        Self {
            id: 0,
            name: path.file_name().unwrap_or_default().to_os_string(),
        }
    }
}
