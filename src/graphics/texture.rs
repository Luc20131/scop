use gl::types::GLint;
use std::{ffi::OsString, path::Path, vec};

use crate::parsers::bmp_parser::{Pixel, image_loader};

#[derive(Debug, Clone)]
pub struct Texture {
    pub id: u32,
    pub name: OsString,
    pub data: Vec<Pixel>,
    pub width: u32,
    pub height: u32,
    pub type_: String,
    is_set: bool,
}

impl Texture {
    pub fn new(path: &Path) -> Self {
        println!("Loading image: {}", path.display());
        // let bmp = image::open(path)
        //     .expect(path.to_str().unwrap_or_default())
        //     .into_rgba8();
        let bmp = image_loader(path);
        match bmp {
            Ok(mut img) => {
                let mut id: u32 = 0;
                unsafe {
                    gl::GenTextures(1, &mut id);
                }
                println!(
                    " file : {}\nbmp width: {}\n bmp height: {}",
                    path.display(),
                    img.width(),
                    img.height()
                );
                return Self {
                    id,
                    name: path.file_name().unwrap_or_default().to_os_string(),
                    width: img.width(),
                    height: img.height(),
                    data: img.img_pixels,
                    type_: "undefined".to_string(),
                    is_set: false,
                };
            }
            Err(error) => {
                dbg!(error);
                println!("Error texture");
                return Self::default();
            }
        }
    }

    pub fn setup_tex(&mut self) {
        if self.is_set {
            return;
        }
        println!("Setup texture {}: {}", self.id, self.name.display());
        self.is_set = true;
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                self.width as i32,
                self.height as i32,
                0,
                gl::BGRA,
                gl::UNSIGNED_BYTE,
                self.data.as_slice().as_ptr() as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_NEAREST as GLint,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
        }
    }
}

impl Default for Texture {
    fn default() -> Self {
        let id: u32 = 0;
        Self {
            id,
            name: OsString::from("Default"),
            width: 1,
            height: 1,
            data: vec![],
            type_: "".to_string(),
            is_set: false,
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}
