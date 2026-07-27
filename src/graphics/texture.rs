use gl::types::GLint;
use image::{EncodableLayout, ImageBuffer, Rgba};
use std::{ffi::OsString, path::Path};

#[derive(Debug, Clone)]
pub struct Texture {
    pub id: u32,
    pub name: OsString,
    pub data: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub width: u32,
    pub height: u32,
    pub type_: String,
}

impl Texture {
    pub fn new(path: &Path) -> Self {
        println!("Loading image: {}", path.display());
        let bmp = image::open(path)
            .expect(path.to_str().unwrap_or_default())
            .into_rgba8();
        let mut id: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        Self {
            id,
            name: path.file_name().unwrap_or_default().to_os_string(),
            width: bmp.width(),
            height: bmp.height(),
            data: bmp,
            type_: "undefined".to_string(),
        }
    }

    pub fn setup_tex(&mut self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            println!("Setup texture {}: {}", self.id, self.name.display());
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                self.width as i32,
                self.height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                self.data.as_bytes().as_ptr() as *const _,
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
            data: ImageBuffer::default(),
            type_: "".to_string(),
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
