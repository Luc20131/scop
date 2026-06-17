use crate::{
    graphics::{mesh::Mesh, texture::Texture},
    parsers::mtl_parser::MtlFile,
};

#[allow(dead_code)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    //TODO faire une map plutot qu'un vec
    pub textures: Vec<Texture>,
    pub mtl_file: Vec<MtlFile>,
}
