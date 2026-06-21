use crate::parsers::bmp_parser::image_loader;
use std::{ffi::OsString, path::Path};

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Texture {
    pub id: u32,
    pub name: OsString,
    pub data: Vec<u8>,
    pub width: i32,
    pub height: i32,
}

impl Texture {
    pub fn new(path: &Path) -> Self {
        let bmp = image_loader(path);
        Self {
            id: 0,
            name: path.file_name().unwrap_or_default().to_os_string(),
            data: bmp.img_pixels,
            width: bmp.img_width,
            height: bmp.img_height,
        }
    }
}

// fn to_gl_color(colors: Vec<u8>) -> Vec<f32> {
//     let mut gl_colors: Vec<f32> = vec![];
//     for color in colors {
//         gl_colors.push(color as f32 * (1.0 / 255.0));
//     }
//     gl_colors
// }
