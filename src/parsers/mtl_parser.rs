use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
    str::SplitWhitespace,
};

use crate::graphics::texture::Texture;

pub const ILLU_MODE_COLOR_ON_AND_AMBIENT_OFF: u8 = 0;
pub const ILLU_MODE_COLOR_ON_AND_AMBIENT_ON: u8 = 1;
pub const ILLU_MODE_HIGHLIGHT_ON: u8 = 2;
pub const ILLU_MODE_REFLECT_RAYTRACE_ON: u8 = 3;
pub const ILLU_MODE_GLASS_REFLECT_RAYTRACE_ON: u8 = 4;
pub const ILLU_MODE_REFLECT_FRESNEL_RAYTRACE_ON: u8 = 5;
pub const ILLU_MODE_REFRACT_RAY_TRACE_ON_FRESNEL_OFF: u8 = 6;
pub const ILLU_MODE_REFRACT_FRESNEL_RAY_TRACE_ON: u8 = 7;
pub const ILLU_MODE_REFLECT_ON_RAY_TRACE_OFF: u8 = 8;
pub const ILLU_MODE_TRANSP_GLASS_ON_RAY_TRACE_OFF: u8 = 9;
pub const ILLU_MODE_CASTS_SHADOWS_INVI_SURFACES: u8 = 10;

#[derive(Debug, Clone)]
pub struct MtlFile {
    pub path: PathBuf,
    pub content: String,
    pub materials: HashMap<String, Material>,
}

#[derive(Debug, Clone, Copy)]
pub struct RGB {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Default for RGB {
    fn default() -> Self {
        Self {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
    }
}

impl RGB {
    pub fn rbg_to_array(&self) -> [f32; 3] {
        [self.red, self.green, self.blue]
    }

    fn set_value_from_words(&mut self, mut words: SplitWhitespace<'_>) {
        self.red = words
            .next()
            .unwrap_or_default()
            .parse::<f32>()
            .unwrap_or(0.0);
        self.green = words
            .next()
            .unwrap_or_default()
            .parse::<f32>()
            .unwrap_or(0.0);
        self.blue = words
            .next()
            .unwrap_or_default()
            .parse::<f32>()
            .unwrap_or(0.0);
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Material {
    pub name: String,
    pub ka: RGB,
    pub kd: RGB,
    pub ks: RGB,
    ns: f32,
    ni: f32,
    d: f32,
    tr: f32,
    tf: RGB,
    illum: usize,
    pub map_ka: Texture,
    pub map_kd: Texture,
    map_ks: Texture,
    pub map_d: Texture,
    pub map_bump: Texture,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            ka: RGB::default(),
            kd: RGB::default(),
            ks: RGB::default(),
            ns: 0.0,
            ni: 0.0,
            d: 1.0,
            tr: 0.0,
            tf: RGB::default(),
            illum: 0,
            map_ka: Texture::default(),
            map_kd: Texture::default(),
            map_ks: Texture::default(),
            map_d: Texture::default(),
            map_bump: Texture::default(),
        }
    }
}

impl Material {
    pub fn ambient_color(&self) -> RGB {
        self.ka
    }

    pub fn diffuse_color(&self) -> RGB {
        self.kd
    }

    pub fn specular_color(&self) -> RGB {
        self.ks
    }

    pub fn init_map(&mut self) {
        if self.map_ka.name != "Default" {
            self.map_ka.setup_tex();
        }
        if self.map_kd.name != "Default" {
            self.map_kd.setup_tex();
        }
        if self.map_ks.name != "Default" {
            self.map_ks.setup_tex();
        }
        if self.map_d.name != "Default" {
            self.map_d.setup_tex();
        }
        if self.map_bump.name != "Default" {
            self.map_bump.setup_tex();
        }
    }
}

impl MtlFile {
    pub fn parse(&mut self) {
        let materials = self.content.split_terminator("newmtl ");

        for elem in materials {
            let mut attributes = elem.lines();
            let mut mtl = Material::default();
            let tmp = attributes.nth(0).unwrap_or_default().to_string();
            if tmp.starts_with("#") {
                drop(mtl);
                continue;
            }
            mtl.name = tmp;
            for attr in attributes {
                let mut words = attr.split_whitespace();
                match words.next().unwrap_or_default() {
                    "Ka" => {
                        mtl.ka.set_value_from_words(words);
                    }
                    "Kd" => {
                        mtl.kd.set_value_from_words(words);
                    }
                    "Ks" => {
                        mtl.ks.set_value_from_words(words);
                    }
                    "Ns" => {
                        mtl.ns = words
                            .next()
                            .unwrap_or_default()
                            .parse::<f32>()
                            .unwrap_or(0.0)
                    }
                    "Ni" => {
                        mtl.ni = words
                            .next()
                            .unwrap_or_default()
                            .parse::<f32>()
                            .unwrap_or(0.0)
                    }
                    "d" => {
                        mtl.d = words
                            .next()
                            .unwrap_or_default()
                            .parse::<f32>()
                            .unwrap_or(0.0);
                        mtl.tr = 1.0 - mtl.d;
                    }
                    "Tr" => {
                        mtl.tr = words
                            .next()
                            .unwrap_or_default()
                            .parse::<f32>()
                            .unwrap_or(0.0);
                        mtl.d = 1.0 - mtl.tr;
                    }
                    "Tf" => {
                        mtl.tf.set_value_from_words(words);
                    }
                    "illum" => {
                        mtl.illum = words
                            .next()
                            .unwrap_or_default()
                            .parse::<usize>()
                            .unwrap_or(0)
                    }
                    "map_d" => {
                        let oui = &words.next().unwrap_or_default();
                        if let Some(parent_file) = self.path.parent() {
                            let path = Path::new(oui);
                            let relative: String = String::from(
                                parent_file.to_str().unwrap_or_default().to_string().clone()
                                    + "/"
                                    + path
                                        .file_name()
                                        .unwrap_or_default()
                                        .to_str()
                                        .unwrap_or_default(),
                            );
                            mtl.map_d = Texture::new(Path::new(&relative));
                            mtl.map_d.type_ = "alpha".to_string();
                        }
                    }
                    "map_Kd" => {
                        let oui = &words.next().unwrap_or_default();
                        if let Some(parent_file) = self.path.parent() {
                            let path = Path::new(oui);
                            let relative: String = String::from(
                                parent_file.to_str().unwrap_or_default().to_string().clone()
                                    + "/"
                                    + path
                                        .file_name()
                                        .unwrap_or_default()
                                        .to_str()
                                        .unwrap_or_default(),
                            );
                            mtl.map_kd = Texture::new(Path::new(&relative));
                            mtl.map_kd.type_ = "diffuse".to_string();
                        }
                    }
                    "map_Ka" => {
                        let oui = &words.next().unwrap_or_default();
                        if let Some(parent_file) = self.path.parent() {
                            let path = Path::new(oui);
                            let relative: String = String::from(
                                parent_file.to_str().unwrap_or_default().to_string().clone()
                                    + "/"
                                    + path
                                        .file_name()
                                        .unwrap_or_default()
                                        .to_str()
                                        .unwrap_or_default(),
                            );
                            mtl.map_ka = Texture::new(Path::new(&relative));
                            mtl.map_ka.type_ = "ambient".to_string();
                        }
                    }
                    "map_Bump" => {
                        let oui = &words.next().unwrap_or_default();
                        if let Some(parent_file) = self.path.parent() {
                            let path = Path::new(oui);
                            let relative: String = String::from(
                                parent_file.to_str().unwrap_or_default().to_string().clone()
                                    + "/"
                                    + path
                                        .file_name()
                                        .unwrap_or_default()
                                        .to_str()
                                        .unwrap_or_default(),
                            );
                            mtl.map_bump = Texture::new(Path::new(&relative));
                            mtl.map_bump.type_ = "normal".to_string();
                        }
                    }
                    _ => {}
                }
            }
            self.materials.insert(mtl.name.clone(), mtl);
        }
    }
}
