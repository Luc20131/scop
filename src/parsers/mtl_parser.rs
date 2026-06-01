use std::{path::PathBuf, str::SplitWhitespace};

pub const IlluMode_ColorOnAndAmbientOff: u8 = 0;
pub const IlluMode_ColorOnAndAmbientOn: u8 = 1;
pub const IlluMode_HighlightOn: u8 = 2;
pub const IlluMode_ReflectRaytraceOn: u8 = 3;
pub const IlluMode_GlassReflectRaytraceOn: u8 = 4;
pub const IlluMode_ReflectFresnelRaytraceOn: u8 = 5;
pub const IlluMode_RefractRayTraceOnFresnelOff: u8 = 6;
pub const IlluMode_RefractFresnelRayTraceOn: u8 = 7;
pub const IlluMode_ReflectOnRayTraceOff: u8 = 8;
pub const IlluMode_TranspGlassOnRayTraceOff: u8 = 9;
pub const IlluMode_CastsShadowsInviSurfaces: u8 = 10;

#[derive(Debug)]
pub struct MtlFile {
    pub path: PathBuf,
    pub content: String,
    pub materials: Vec<Material>,
}

#[derive(Debug)]
struct RGB {
    red: f32,
    green: f32,
    blue: f32,
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

#[derive(Debug)]
pub struct Material {
    pub name: String,
    ka: RGB,
    kd: RGB,
    ks: RGB,
    ns: f32,
    ni: f32,
    d: f32,
    tr: f32,
    tf: RGB,
    illum: usize,
    map_kd: PathBuf,
    map_ks: PathBuf,
    map_d: PathBuf,
    map_bump: PathBuf,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name: String::default(),
            ka: RGB::default(),
            kd: RGB::default(),
            ks: RGB::default(),
            ns: 0.0,
            ni: 0.0,
            d: 1.0,
            tr: 0.0,
            tf: RGB::default(),
            illum: 0,
            map_kd: PathBuf::default(),
            map_ks: PathBuf::default(),
            map_d: PathBuf::default(),
            map_bump: PathBuf::default(),
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
                        mtl.illum = (words
                            .next()
                            .unwrap_or_default()
                            .parse::<usize>()
                            .unwrap_or(0))
                    }
                    _ => {}
                }
            }
            self.materials.push(mtl);
        }
    }
}
