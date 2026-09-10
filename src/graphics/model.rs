use std::collections::HashMap;

use crate::{
    graphics::{mesh::Mesh, shaders::Shader, texture::Texture},
    math::vec3::Vec3,
    parsers::mtl_parser::{Material, MtlFile},
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub textures: Vec<Texture>,
    pub mtl_file: Vec<MtlFile>,
    pub materials: HashMap<String, Material>,
    pub render_type: u32,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            meshes: vec![],
            textures: vec![],
            mtl_file: vec![],
            render_type: gl::TRIANGLES,
            materials: HashMap::new(),
        }
    }
}

impl Model {
    pub fn draw(&self, shader: &mut Shader, cam_pos: Vec3) {
        for mesh in &self.meshes {
            mesh.draw(self.render_type, shader, cam_pos.clone());
        }
    }
}
