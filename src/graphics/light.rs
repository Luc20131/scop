use crate::{
    graphics::{gl_wrapper::Vao, shaders::Shader},
    math::vec3::Vec3,
};

#[derive(Debug, Clone)]
pub struct Light {
    color: Vec3,
    pos: Vec3,
    vao: Vao,
    shader: Shader,
}

impl Light {
    pub fn new(shader: Shader) -> Self {
        let res = Self {
            color: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            pos: Vec3::new(0.0, 0.0, 0.0),
            vao: Vao::new(),
            shader: shader,
        };
        res.vao.bind();
        res
    }
}
