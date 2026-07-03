use std::{mem, ptr};

use gl::types::{GLfloat, GLsizei};

use crate::{
    graphics::{
        gl_wrapper::{BufferObject, Vao, VertexAttribute},
        shaders::Shader,
    },
    math::vec3::Vec3,
};

#[derive(Debug, Clone)]
pub struct Light {
    color: [f32; 3],
    pos: Vec3,
    vao: Vao,
    vbo: BufferObject,
    shader: Shader,
}

impl Light {
    pub fn new(shader: Shader) -> Self {
        let res = Self {
            color: [1.0; 3],

            pos: Vec3::new(100.0, 100.0, 100.0),
            vao: Vao::new(),
            vbo: BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW),
            shader: shader,
        };
        res.vao.bind();
        res.vbo.bind();
        let tmp = [
            res.pos.x, res.pos.y, res.pos.z, res.pos.x, res.pos.y, res.pos.z, res.pos.x, res.pos.y,
            res.pos.z,
        ];
        res.vbo.store_f32_data(&tmp);

        let position_attribute = VertexAttribute::new(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            9 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
        position_attribute.enable();

        res.vao.unbind();
        res.vbo.unbind();
        res
    }

    pub fn draw(&mut self) {
        self.shader.use_prog();
        self.vao.bind();
        // self.shader.set_vec3("objectColor", &[0.31, 0.31, 0.56]);
        self.shader.set_vec3("ligthColor", self.color.as_slice());
        self.shader
            .set_vec3("lightPos", &[self.pos.x, self.pos.y, self.pos.z]);
        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::POINT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        self.vao.unbind();
    }
}
