use image::{EncodableLayout, ImageBuffer, Rgba};
use std::{ffi::OsString, path::Path};

#[derive(Debug, Clone)]
pub struct Texture {
    pub id: u32,
    pub name: OsString,
    pub data: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub width: u32,
    pub height: u32,
    pub _type: String,
}

impl Texture {
    pub fn new(path: &Path) -> Self {
        let bmp = image::open(path).expect("crash").into_rgba8();
        let mut id: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        Self {
            id: id,
            name: path.file_name().unwrap_or_default().to_os_string(),
            width: bmp.width(),
            height: bmp.height(),
            data: bmp,
            _type: "".to_string(),
        }
    }

    pub fn setup_tex(&mut self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
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
