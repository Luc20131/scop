use crate::{
    graphics::{mesh::Mesh, texture::Texture},
    parsers::mtl_parser::MtlFile,
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub textures: Vec<Texture>,
    pub mtl_file: Vec<MtlFile>,
    pub render_type: u32,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            meshes: vec![],
            textures: vec![],
            mtl_file: vec![],
            render_type: gl::TRIANGLES,
        }
    }
}

impl Model {
    pub fn draw(&self) {
        for mesh in &self.meshes {
            mesh.draw(self.render_type);
        }
    }
}
