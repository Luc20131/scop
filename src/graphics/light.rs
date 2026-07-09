use std::path::Path;

use crate::{
    graphics::{mesh::Mesh, shaders::Shader},
    math::vec3::Vec3,
    parsers::{mtl_parser::RGB, obj_parser::ObjFile},
};

#[derive(Debug, Clone)]
pub struct Light {
    color: RGB,
    mesh: Mesh,
    pub pos: Vec3,
    shader: Shader,
}

impl Light {
    pub fn new(shader: Shader) -> Self {
        let mut res = Self {
            color: RGB::default(),
            pos: Vec3::new(20.0, 20.0, 20.0),
            mesh: Mesh::default(),
            shader,
        };

        res.mesh = ObjFile::new(Path::new("./resources/triangle.obj"))
            .unwrap()
            .model
            .meshes[0]
            .clone();
        res.mesh.setup_mesh();
        res
    }

    pub fn draw(&mut self, cam_pos: Vec3) {
        self.mesh.draw(gl::TRIANGLES, &mut self.shader, cam_pos);
    }

    pub fn color(&self) -> RGB {
        self.color
    }

    pub fn change_light_color(&mut self, color: RGB) {
        self.color = color;
    }
}
