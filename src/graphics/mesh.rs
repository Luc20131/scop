use std::fs;

use crate::graphics::gl_wrapper::{BufferObject, Vao};
use crate::graphics::texture::Texture;
use crate::my_lib::vec3::Vec3;

pub type Normal = Vec3;

pub struct Vertex {
    position: Vec3,
    normals: Vec3,
    tex_coord: Vec3,
}

pub struct Mesh {
    pub name: String,
    pub vertices: Vec<Vertex>,
    pub normal: Vec<Normal>,
    pub texture: Vec<Texture>,
    vao: Vao,
    vbo: BufferObject,
    ebo: BufferObject,
}

impl Mesh {
    pub fn new(
        name: String,   
        vertices: Vec<Vertex>,
        normal: Vec<Normal>,
        texture: Vec<Texture>,
    ) -> Self {
        Self {
            name,
            vertices,
            normal,
            texture,
            vao: Vao::new(),
            vbo: BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW),
            ebo: BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW),
        }
    }

    fn setup_mesh(&mut self) {
        self.vao.bind();
        self.vbo.bind();
        self.vbo
            .store_f32_data(&(self.pos_vertices_slice().as_slice()));
        self.ebo.bind();
    }

    fn pos_vertices_slice(&self) -> Vec<f32> {
        let mut pos: Vec<f32> = Vec::new();
        for elem in self.vertices.iter() {
            pos.push(elem.position.x);
            pos.push(elem.position.y);
            pos.push(elem.position.z);
        }
        pos
    }
}
