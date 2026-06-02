use crate::graphics::{mesh::Mesh, texture::Texture};

#[allow(dead_code)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub textures: Vec<Texture>,
}
