use std::path::PathBuf;

#[derive(Debug)]
#[allow(dead_code)]
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

#[repr(u8)]
#[derive(Debug)]
enum IlluMode {
    ColorOnAndAmbientOff,
    ColorOnAndAmbientOn,
    HighlightOn,
    ReflectionRaytraceOn,
    GlassReflectionRaytraceOn,
    ReflectionFresnelRaytraceOn,
    RefractionRayTraceOnFresnelOff,
    RefractionFresnelRayTraceOn,
    ReflectionOnRayTraceOff,
    TranspGlassOnRayTraceOff,
    CastsShadowsInviSurfaces,
}

#[derive(Debug)]
pub struct Material {
    name: String,
    ka: RGB,
    kd: RGB,
    ks: RGB,
    ns: f32,
    ni: f32,
    tr: f32,
    tf: RGB,
    illum: IlluMode,
    map_kd: PathBuf,
    map_ks: PathBuf,
    map_d: PathBuf,
    map_bump: PathBuf,
}

#[allow(dead_code)]
impl MtlFile {
    pub fn parse() {
        todo!("implemente mtl parsing");
    }
}
